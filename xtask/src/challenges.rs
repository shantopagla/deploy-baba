//! Challenges content sync — `content/challenges/*.md` is the source of truth (ADR-036).
//!
//! SQLite's `challenges` table is a read-optimized cache. `content_sha` (a hash of the raw
//! markdown bytes) is the freshness signal that lets these commands tell whether a file is
//! "newer than the database":
//!
//! - `sync`: upsert changed files + prune removed ones directly into a SQLite file. Used for
//!   local dev DBs and ahead of RAG ingest (disposable/regenerable).
//! - `diff`: read-only drift report; exits non-zero if content/ and the DB disagree. Used as
//!   a CI/quality gate so drift can't be merged silently.
//! - `gen-migration`: diffs content/ against a fresh migrations-only DB and emits a normal
//!   ADR-010 migration file for just what changed. This is the only channel that reaches
//!   production (EFS SQLite only gets new content via migrations applied at Lambda cold
//!   start, per ADR-002/ADR-010).

use anyhow::{Context, Result};
use clap::Subcommand;
use rusqlite::{Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Subcommand)]
pub enum ChallengesAction {
    /// Sync content/challenges/*.md into SQLite (upsert changed, prune removed).
    Sync {
        #[arg(long, default_value = "deploy-baba.db")]
        db_path: PathBuf,
        #[arg(long, default_value = "content/challenges")]
        content_dir: PathBuf,
    },
    /// Report drift between content/challenges/*.md and SQLite without writing (CI gate).
    Diff {
        #[arg(long, default_value = "deploy-baba.db")]
        db_path: PathBuf,
        #[arg(long, default_value = "content/challenges")]
        content_dir: PathBuf,
    },
    /// Generate a migration file for what changed in content/challenges/*.md since the last
    /// generated migration (diffs against a fresh migrations-only DB, not the live dev DB).
    GenMigration {
        #[arg(long, default_value = "content/challenges")]
        content_dir: PathBuf,
        #[arg(long, default_value = "services/ui/migrations")]
        migrations_dir: PathBuf,
    },
}

pub async fn execute(action: ChallengesAction) -> Result<()> {
    match action {
        ChallengesAction::Sync {
            db_path,
            content_dir,
        } => sync_cmd(&db_path, &content_dir),
        ChallengesAction::Diff {
            db_path,
            content_dir,
        } => diff_cmd(&db_path, &content_dir),
        ChallengesAction::GenMigration {
            content_dir,
            migrations_dir,
        } => gen_migration_cmd(&content_dir, &migrations_dir),
    }
}

// ── Parsing ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
struct Frontmatter {
    slug: String,
    title: String,
    #[serde(default)]
    job_slug: Option<String>,
    #[serde(default)]
    short_description: Option<String>,
    #[serde(default)]
    tech_stack: Vec<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    image_url: Option<String>,
    #[serde(default)]
    featured: bool,
    #[serde(default)]
    sort_order: i64,
    #[serde(default)]
    related_plan_module: Option<String>,
    #[serde(default)]
    related_adr: Option<String>,
}

#[derive(Debug, Clone)]
struct Challenge {
    fm: Frontmatter,
    description: String,
    problem: Option<String>,
    constraints: Option<String>,
    decisions: Option<String>,
    implementation: Option<String>,
    outcomes: Option<String>,
    metrics: Option<String>,
    content_sha: String,
    source_path: PathBuf,
}

fn parse_file(path: &Path) -> Result<Challenge> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    let content_sha = format!("{:x}", hasher.finalize());

    let (fm_str, body) = split_frontmatter(&raw).with_context(|| {
        format!(
            "{}: missing YAML frontmatter (expected a leading `---` block)",
            path.display()
        )
    })?;
    let fm: Frontmatter = serde_yaml::from_str(fm_str)
        .with_context(|| format!("{}: invalid frontmatter", path.display()))?;

    let sections = split_headings(body);
    let description = sections
        .get("description")
        .cloned()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("{}: missing `## Description` section", path.display()))?;

    Ok(Challenge {
        problem: sections.get("problem").cloned(),
        constraints: sections.get("constraints").cloned(),
        decisions: sections.get("decisions").cloned(),
        implementation: sections.get("implementation").cloned(),
        outcomes: sections.get("outcomes").cloned(),
        metrics: sections.get("metrics").cloned(),
        description,
        fm,
        content_sha,
        source_path: path.to_path_buf(),
    })
}

/// Split `---\n<yaml>\n---\n<body>` into `(yaml, body)`.
fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
    let rest = raw.strip_prefix("---")?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let end = rest.find("\n---")?;
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after.strip_prefix('\n').unwrap_or(after);
    Some((fm, body))
}

/// Split a markdown body into `## Heading` sections, keyed by lowercased heading text.
fn split_headings(body: &str) -> BTreeMap<String, String> {
    let mut sections: BTreeMap<String, String> = BTreeMap::new();
    let mut current_key: Option<String> = None;
    let mut current_text = String::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(key) = current_key.take() {
                sections.insert(key, current_text.trim().to_string());
            }
            current_key = Some(heading.trim().to_lowercase());
            current_text.clear();
        } else if current_key.is_some() {
            current_text.push_str(line);
            current_text.push('\n');
        }
    }
    if let Some(key) = current_key {
        sections.insert(key, current_text.trim().to_string());
    }
    sections
}

fn load_all(content_dir: &Path) -> Result<Vec<Challenge>> {
    let mut out = Vec::new();
    if !content_dir.exists() {
        return Ok(out);
    }
    let mut entries: Vec<PathBuf> = std::fs::read_dir(content_dir)
        .with_context(|| format!("failed to read {}", content_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("md"))
        .collect();
    entries.sort();

    for path in entries {
        out.push(parse_file(&path)?);
    }
    out.sort_by(|a, b| a.fm.slug.cmp(&b.fm.slug));
    Ok(out)
}

// ── Schema helpers ───────────────────────────────────────────────────────────

fn has_column(conn: &Connection, table: &str, column: &str) -> bool {
    let pragma = format!("PRAGMA table_info({table})");
    let Ok(mut stmt) = conn.prepare(&pragma) else {
        return false;
    };
    let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(1)) else {
        return false;
    };
    let cols: Vec<String> = rows.filter_map(|r| r.ok()).collect();
    cols.iter().any(|c| c == column)
}

/// Ensure the `challenges` table (022) and `content_sha` column (038) exist. Self-heals the
/// column defensively so `sync`/`diff` work even against a DB whose migration runner hasn't
/// caught up yet (mirrors migration 038's own `ALTER TABLE`).
fn ensure_schema(conn: &Connection) -> Result<()> {
    let table_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='challenges'",
        [],
        |row| row.get(0),
    )?;
    if table_exists == 0 {
        anyhow::bail!(
            "`challenges` table not found — run `just dev-stack` (or apply migrations) at least once before syncing content"
        );
    }
    if !has_column(conn, "challenges", "content_sha") {
        conn.execute("ALTER TABLE challenges ADD COLUMN content_sha TEXT", [])?;
    }
    Ok(())
}

fn existing_content_sha(conn: &Connection, slug: &str) -> Result<Option<String>> {
    Ok(conn
        .query_row(
            "SELECT content_sha FROM challenges WHERE slug = ?1",
            [slug],
            |row| row.get::<_, Option<String>>(0),
        )
        .optional()?
        .flatten())
}

fn db_slugs(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT slug FROM challenges")?;
    let slugs = stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(slugs)
}

// ── sync ─────────────────────────────────────────────────────────────────────

fn sync_cmd(db_path: &Path, content_dir: &Path) -> Result<()> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("failed to open {}", db_path.display()))?;
    ensure_schema(&conn)?;

    let challenges = load_all(content_dir)?;
    let mut synced = 0u32;
    let mut skipped = 0u32;

    for c in &challenges {
        if existing_content_sha(&conn, &c.fm.slug)?.as_deref() == Some(c.content_sha.as_str()) {
            skipped += 1;
            continue;
        }
        upsert(&conn, c)?;
        synced += 1;
    }

    let keep: Vec<String> = challenges.iter().map(|c| c.fm.slug.clone()).collect();
    let pruned = prune(&conn, &keep)?;

    println!("challenges sync: {synced} synced, {skipped} skipped, {pruned} pruned");
    Ok(())
}

fn upsert(conn: &Connection, c: &Challenge) -> Result<()> {
    let job_id: Option<i64> = match c.fm.job_slug.as_deref() {
        Some(slug) if !slug.is_empty() => conn
            .query_row("SELECT id FROM jobs WHERE slug = ?1", [slug], |row| {
                row.get(0)
            })
            .optional()?,
        _ => None,
    };
    let tech_stack = c.fm.tech_stack.join(",");

    conn.execute(
        "INSERT INTO challenges (
            slug, title, job_id, description, short_description, tech_stack, category, url, image_url,
            problem, constraints, decisions, implementation, outcomes, metrics,
            related_job_slug, related_plan_module, related_adr, featured, sort_order, content_sha
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21)
         ON CONFLICT(slug) DO UPDATE SET
            title = excluded.title,
            job_id = excluded.job_id,
            description = excluded.description,
            short_description = excluded.short_description,
            tech_stack = excluded.tech_stack,
            category = excluded.category,
            url = excluded.url,
            image_url = excluded.image_url,
            problem = excluded.problem,
            constraints = excluded.constraints,
            decisions = excluded.decisions,
            implementation = excluded.implementation,
            outcomes = excluded.outcomes,
            metrics = excluded.metrics,
            related_job_slug = excluded.related_job_slug,
            related_plan_module = excluded.related_plan_module,
            related_adr = excluded.related_adr,
            featured = excluded.featured,
            sort_order = excluded.sort_order,
            content_sha = excluded.content_sha",
        rusqlite::params![
            c.fm.slug,
            c.fm.title,
            job_id,
            c.description,
            c.fm.short_description,
            if tech_stack.is_empty() {
                None
            } else {
                Some(tech_stack)
            },
            c.fm.category,
            c.fm.url,
            c.fm.image_url,
            c.problem,
            c.constraints,
            c.decisions,
            c.implementation,
            c.outcomes,
            c.metrics,
            c.fm.job_slug,
            c.fm.related_plan_module,
            c.fm.related_adr,
            c.fm.featured as i64,
            c.fm.sort_order,
            c.content_sha,
        ],
    )?;
    Ok(())
}

fn prune(conn: &Connection, keep_slugs: &[String]) -> Result<u32> {
    let mut pruned = 0u32;
    for slug in db_slugs(conn)? {
        if !keep_slugs.contains(&slug) {
            conn.execute("DELETE FROM challenges WHERE slug = ?1", [&slug])?;
            pruned += 1;
        }
    }
    Ok(pruned)
}

// ── diff ─────────────────────────────────────────────────────────────────────

struct Drift {
    new: Vec<String>,
    changed: Vec<String>,
    removed: Vec<String>,
}

impl Drift {
    fn is_empty(&self) -> bool {
        self.new.is_empty() && self.changed.is_empty() && self.removed.is_empty()
    }
}

fn compute_drift(conn: &Connection, challenges: &[Challenge]) -> Result<Drift> {
    let mut new = Vec::new();
    let mut changed = Vec::new();

    for c in challenges {
        match existing_content_sha(conn, &c.fm.slug)? {
            None => new.push(c.fm.slug.clone()),
            Some(sha) if sha != c.content_sha => changed.push(c.fm.slug.clone()),
            _ => {}
        }
    }

    let content_slugs: Vec<String> = challenges.iter().map(|c| c.fm.slug.clone()).collect();
    let removed: Vec<String> = db_slugs(conn)?
        .into_iter()
        .filter(|s| !content_slugs.contains(s))
        .collect();

    Ok(Drift {
        new,
        changed,
        removed,
    })
}

fn diff_cmd(db_path: &Path, content_dir: &Path) -> Result<()> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("failed to open {}", db_path.display()))?;
    ensure_schema(&conn)?;

    let challenges = load_all(content_dir)?;
    let drift = compute_drift(&conn, &challenges)?;

    if drift.is_empty() {
        println!("challenges diff: no drift — content/challenges/*.md matches the database.");
        return Ok(());
    }

    println!("challenges diff: drift detected");
    if !drift.new.is_empty() {
        println!("  new:     {}", drift.new.join(", "));
    }
    if !drift.changed.is_empty() {
        println!("  changed: {}", drift.changed.join(", "));
    }
    if !drift.removed.is_empty() {
        println!("  removed: {}", drift.removed.join(", "));
    }
    println!("Run `just challenges-migration` to generate a migration for these changes.");
    std::process::exit(1);
}

// ── gen-migration ────────────────────────────────────────────────────────────

fn gen_migration_cmd(content_dir: &Path, migrations_dir: &Path) -> Result<()> {
    let tmp_db = std::env::temp_dir().join(format!(
        "challenges-gen-migration-{}.db",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&tmp_db);

    let result = (|| -> Result<()> {
        let conn = Connection::open(&tmp_db)
            .with_context(|| format!("failed to open temp db {}", tmp_db.display()))?;

        let mut files: Vec<PathBuf> = std::fs::read_dir(migrations_dir)
            .with_context(|| format!("failed to read {}", migrations_dir.display()))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("sql"))
            .collect();
        files.sort();
        for f in &files {
            let sql = std::fs::read_to_string(f)
                .with_context(|| format!("failed to read {}", f.display()))?;
            conn.execute_batch(&sql)
                .with_context(|| format!("failed to apply {}", f.display()))?;
        }
        ensure_schema(&conn)?;

        let challenges = load_all(content_dir)?;
        let drift = compute_drift(&conn, &challenges)?;

        if drift.is_empty() {
            println!("challenges gen-migration: no drift — nothing to generate.");
            return Ok(());
        }

        let changed: Vec<&Challenge> = challenges
            .iter()
            .filter(|c| drift.new.contains(&c.fm.slug) || drift.changed.contains(&c.fm.slug))
            .collect();

        let date = current_date()?;
        let next_num = next_migration_number(migrations_dir)?;
        let file_stub = format!("{next_num:03}_challenges_content_{date}");
        let file_path = migrations_dir.join(format!("{file_stub}.sql"));

        let sql = render_migration_sql(&changed, &drift.removed);
        std::fs::write(&file_path, sql)
            .with_context(|| format!("failed to write {}", file_path.display()))?;

        println!("Generated {}", file_path.display());
        println!(
            "  {} new/changed, {} removed",
            changed.len(),
            drift.removed.len()
        );
        println!(
            "Register \"{file_stub}\" in services/ui/src/db.rs MIGRATIONS array (see /add-migration)."
        );
        Ok(())
    })();

    let _ = std::fs::remove_file(&tmp_db);
    let _ = std::fs::remove_file(format!("{}-wal", tmp_db.display()));
    let _ = std::fs::remove_file(format!("{}-shm", tmp_db.display()));

    result
}

fn next_migration_number(migrations_dir: &Path) -> Result<u32> {
    let mut max = 0u32;
    for entry in std::fs::read_dir(migrations_dir)
        .with_context(|| format!("failed to read {}", migrations_dir.display()))?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if let Some(prefix) = name.split('_').next() {
            if let Ok(n) = prefix.parse::<u32>() {
                max = max.max(n);
            }
        }
    }
    Ok(max + 1)
}

fn current_date() -> Result<String> {
    let output = std::process::Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .context("failed to run `date`")?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn escape_sql(s: &str) -> String {
    s.replace('\'', "''")
}

fn sql_str_or_null(v: &Option<String>) -> String {
    match v {
        Some(s) if !s.is_empty() => format!("'{}'", escape_sql(s)),
        _ => "NULL".to_string(),
    }
}

/// Render ADR-010 upsert blocks for `changed` + prune `DELETE`s for `removed`, matching the
/// hand-written shape of migrations 022/030 (job_id resolved via a `jobs.slug` subquery so the
/// migration is portable across environments with different auto-increment IDs).
fn render_migration_sql(changed: &[&Challenge], removed: &[String]) -> String {
    let mut out = String::new();
    out.push_str(
        "-- Generated by `just challenges-migration` (xtask challenges gen-migration) from\n\
         -- content/challenges/*.md. content/challenges/*.md is the source of truth (ADR-036) —\n\
         -- do not hand-edit challenge rows here; edit the source .md file and regenerate.\n\n",
    );

    if !removed.is_empty() {
        out.push_str("-- Removed from content/challenges/ since the last generated migration.\n");
        for slug in removed {
            out.push_str(&format!(
                "DELETE FROM challenges WHERE slug = '{}';\n",
                escape_sql(slug)
            ));
        }
        out.push('\n');
    }

    for c in changed {
        out.push_str(&render_upsert(c));
        out.push('\n');
    }

    out
}

fn render_upsert(c: &Challenge) -> String {
    let job_id_expr = match c.fm.job_slug.as_deref() {
        Some(slug) if !slug.is_empty() => {
            format!("(SELECT id FROM jobs WHERE slug = '{}')", escape_sql(slug))
        }
        _ => "NULL".to_string(),
    };
    let tech_stack = c.fm.tech_stack.join(",");
    let tech_stack_sql = if tech_stack.is_empty() {
        "NULL".to_string()
    } else {
        format!("'{}'", escape_sql(&tech_stack))
    };

    format!(
        "-- source: {source}\n\
         INSERT INTO challenges (slug, title, job_id, description, short_description, tech_stack, category, url, image_url,\n\
         \x20                    problem, constraints, decisions, implementation, outcomes, metrics,\n\
         \x20                    related_job_slug, related_plan_module, related_adr, featured, sort_order, content_sha)\n\
         VALUES ('{slug}', '{title}', {job_id_expr}, '{description}', {short_description}, {tech_stack_sql},\n\
         \x20       {category}, {url}, {image_url}, {problem}, {constraints}, {decisions}, {implementation},\n\
         \x20       {outcomes}, {metrics}, {related_job_slug}, {related_plan_module}, {related_adr}, {featured}, {sort_order}, '{content_sha}')\n\
         ON CONFLICT(slug) DO UPDATE SET\n\
         \x20   title = excluded.title,\n\
         \x20   job_id = excluded.job_id,\n\
         \x20   description = excluded.description,\n\
         \x20   short_description = excluded.short_description,\n\
         \x20   tech_stack = excluded.tech_stack,\n\
         \x20   category = excluded.category,\n\
         \x20   url = excluded.url,\n\
         \x20   image_url = excluded.image_url,\n\
         \x20   problem = excluded.problem,\n\
         \x20   constraints = excluded.constraints,\n\
         \x20   decisions = excluded.decisions,\n\
         \x20   implementation = excluded.implementation,\n\
         \x20   outcomes = excluded.outcomes,\n\
         \x20   metrics = excluded.metrics,\n\
         \x20   related_job_slug = excluded.related_job_slug,\n\
         \x20   related_plan_module = excluded.related_plan_module,\n\
         \x20   related_adr = excluded.related_adr,\n\
         \x20   featured = excluded.featured,\n\
         \x20   sort_order = excluded.sort_order,\n\
         \x20   content_sha = excluded.content_sha;\n",
        source = c.source_path.display(),
        slug = escape_sql(&c.fm.slug),
        title = escape_sql(&c.fm.title),
        job_id_expr = job_id_expr,
        description = escape_sql(&c.description),
        short_description = sql_str_or_null(&c.fm.short_description),
        tech_stack_sql = tech_stack_sql,
        category = sql_str_or_null(&c.fm.category),
        url = sql_str_or_null(&c.fm.url),
        image_url = sql_str_or_null(&c.fm.image_url),
        problem = sql_str_or_null(&c.problem),
        constraints = sql_str_or_null(&c.constraints),
        decisions = sql_str_or_null(&c.decisions),
        implementation = sql_str_or_null(&c.implementation),
        outcomes = sql_str_or_null(&c.outcomes),
        metrics = sql_str_or_null(&c.metrics),
        related_job_slug = sql_str_or_null(&c.fm.job_slug),
        related_plan_module = sql_str_or_null(&c.fm.related_plan_module),
        related_adr = sql_str_or_null(&c.fm.related_adr),
        featured = c.fm.featured as i64,
        sort_order = c.fm.sort_order,
        content_sha = c.content_sha,
    )
}

// ── Reusable by other xtask modules (e.g. rag.rs ingest()) ─────────────────

/// Sync content/challenges/*.md into the given DB. Public so `rag ingest` can call it before
/// chunking the portfolio corpus, guaranteeing RAG never indexes stale challenge prose.
pub fn sync(db_path: &Path, content_dir: &Path) -> Result<()> {
    sync_cmd(db_path, content_dir)
}
