# ADR-036: Challenges Content as Markdown Source of Truth

**Date:** 2026-07-08
**Status:** Accepted
**Affected modules:** W-CHL, W-RAG, W-SYNC, W-UI, W-WEB

---

## Context

Challenges/projects content (W-CHL) lived only in SQLite: seeded once via
`services/ui/migrations/022_create_challenges.sql` + `030_challenge_rag_shape.sql`, then
mutated live through Cognito-gated dashboard CRUD
(`POST/PUT/DELETE /api/admin/challenges`). Any dashboard edit had to be manually bridged
back into a source migration via the `sync-dashboard-data` skill (W-SYNC, ADR-010) or it
was lost on the next fresh deploy — SQLite was the de facto source of truth, and the repo
was a lagging, manually-reconciled mirror.

This is inverted for challenges specifically. Jobs, competencies, about-sections, and
social-links are unaffected and keep their existing DB + dashboard + W-SYNC flow.

Production SQLite on EFS (ADR-002) only ever receives new content two ways: migrations
applied idempotently at Lambda cold start, or live dashboard writes. Since dashboard
writes are being turned off for challenges, migrations remain the only channel into
production.

---

## Decision

> `content/challenges/*.md` is the source of truth for challenge content. SQLite's
> `challenges` table is a read-optimized cache, refreshed from disk whenever the
> repo content is newer.

- Each challenge is one file: `content/challenges/<slug>.md`, with YAML frontmatter for
  scalar/list fields (`title`, `job_slug`, `short_description`, `tech_stack`, `category`,
  `url`, `image_url`, `featured`, `sort_order`, `related_plan_module`, `related_adr`) and
  `## Heading` sections in the markdown body for long-text fields (`description`,
  `problem`, `constraints`, `decisions`, `implementation`, `outcomes`, `metrics`).
- A new nullable `content_sha` column (migration 038) holds a SHA-256 hash of the raw
  `.md` file bytes — the freshness signal used to detect "repo is newer than DB."
- `xtask challenges` (invoked via `just challenges-sync` / `just challenges-diff` /
  `just challenges-migration`) provides three operations:
  - `sync` — direct ADR-010 upsert of changed files + prune of removed slugs into a given
    SQLite file. Used for local dev DBs (`just dev-stack`) and ahead of RAG ingest
    (`xtask rag ingest`), both disposable/regenerable.
  - `diff` — read-only drift report; exits non-zero if `content/challenges/*.md` and the
    DB disagree. Used as a CI gate (`.github/workflows/ci.yml` `challenges` job) so drift
    can't be merged silently.
  - `gen-migration` — diffs `content/challenges/*.md` against a **fresh migrations-only**
    DB (not the live dev DB) and emits a normal ADR-010 migration file for just what
    changed. This is the only channel that reaches production, reusing the existing
    migration-runner-at-cold-start mechanism rather than inventing a new Lambda action or
    S3 push.
- `POST/PUT/DELETE /api/admin/challenges*` are removed. The dashboard's Challenges pages
  become read-only, pointing at the source `.md` file and `just challenges-migration`.
- Deleting a `.md` file prunes the corresponding SQLite row automatically the next time
  `sync` runs (full slug set on disk is authoritative).
- Migrations 022/030 are left untouched (migrations are immutable history) and continue to
  act as the bootstrap seed for a brand-new empty DB; generated content migrations layer
  on top and always win for any slug present in `content/challenges/`.

---

## Consequences

### Positive

- Challenge content is reviewable, diffable, and versioned in git like any other source
  file — no more manual DB → migration reconciliation for this content type.
- RAG ingest (`xtask rag ingest`) never indexes stale challenge prose: `ingest()` calls
  `challenges::sync()` before chunking the portfolio corpus.
- No new AWS resources or Lambda actions — production delivery reuses the existing
  migration-at-cold-start mechanism (ADR-002/ADR-010).
- `content_sha` makes `sync`/`diff` idempotent and cheap to re-run.

### Negative / Trade-offs

- Two write paths now exist across the portfolio content types: jobs/competencies/
  about/social-links still go DB-first via dashboard + W-SYNC; challenges go
  repo-first via `.md` + `gen-migration`. Contributors must know which applies where.
- A developer who edits `content/challenges/*.md` and forgets to run
  `just challenges-migration` will have their change caught by CI (`challenges diff`)
  but not automatically published — an explicit step is still required.
- Dashboard users lose the ability to quickly tweak challenge copy without a local dev
  environment and a commit.

### Neutral

- `xtask challenges gen-migration` mirrors the `sync-dashboard-data` skill's Pull → Diff →
  Author → Verify workflow, just inverted in direction (repo → DB instead of DB → repo).

---

## Alternatives Considered

| Option | Rejected because |
|--------|-----------------|
| New Lambda action (`{"action":"sync-challenges"}`) + bundle `content/challenges/*.md` into the Lambda zip, mirroring `rag-push`/`ingest-rag` | Extra AWS surface and a new bundling step for a problem migrations already solve; rejected per "boring infrastructure" (ADR-002 spirit) |
| Push local SQLite file straight to prod EFS | No existing channel does this; would require a new admin action and bypasses the migration audit trail |
| Keep dashboard CRUD writable, `.md` sync runs are advisory only | Reintroduces the exact drift problem this ADR fixes — two writable sources of truth |
| Track a per-row "dashboard-owned" flag so both `.md` and dashboard writes coexist | More logic for a case the plan didn't need; dashboard editing wasn't a hard requirement once `.md` is authoritative |

---

## Cross-References

- → ADR-002 (SQLite on EFS + S3 backup — migrations are the only channel into prod)
- → ADR-010 (SQLite upsert re-seed convention — reused verbatim by `gen-migration`)
- → ADR-016 (RAG Architecture — challenges as the 7th corpus; `ingest()` now syncs first)
- → W-CHL (module plan — challenges feature, updated with the markdown SSOT pivot)
- → W-SYNC (dashboard → migrations sync workflow — inverse direction, same upsert pattern)
- → W-WEB (dashboard Challenges/ChallengeDetail pages made read-only)
