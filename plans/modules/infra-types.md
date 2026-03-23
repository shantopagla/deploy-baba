# W-INFR: infra-types
**Crate:** `crates/infra-types/` | **Status:** DONE
**Coverage floor:** 75% | **Depends on:** W-CFG (optional feature), serde | **Depended on by:** W-UI, W-XT

---

## W-INFR.1 Purpose

Shared Rust types for infrastructure configuration. Defines `SqliteConfig`,
`S3BackupConfig`, and stack-level types that `services/ui/` and `xtask/`
both consume. Sanitized from `services/baba-stack` in `~/shanto`.

PostgreSQL/MySQL engine types were dropped entirely. → ADR-002

---

## W-INFR.2 Public API Surface

```rust
/// SQLite database configuration with optional S3 backup.
/// This is the only database type supported in deploy-baba.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    /// Filesystem path e.g. "/mnt/db/deploy-baba.db"
    pub path: PathBuf,
    /// S3 backup config. None = no automatic backup.
    pub backup: Option<S3BackupConfig>,
    /// WAL mode enabled by default for concurrent reads
    #[serde(default = "default_wal")]
    pub wal_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3BackupConfig {
    pub bucket: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    #[serde(default = "default_retention")]
    pub retain_versions: u32,
    /// EventBridge cron format, e.g. "rate(1 day)"
    #[serde(default = "default_schedule")]
    pub schedule: String,
}

/// Top-level stack configuration (mirrors stack.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackConfig {
    pub deploy: DeployConfig,
    pub database: SqliteConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    pub mode: DeployMode,
}

pub enum DeployMode { Lambda, EcsFargateSpot }
```

---

## W-INFR.3 Implementation Notes

- Source: sanitized from `~/shanto/services/baba-stack`
- All Baba-internal fields removed; no SSO/auth types
- `config-core` dependency is optional (behind feature flag) — infra-types can be used
  without the full config layer
- `stack.toml` maps directly to `StackConfig` struct

`stack.toml` example:
```toml
[deploy]
mode = "lambda"

[database]
path = "/mnt/db/deploy-baba.db"
wal_mode = true

[database.backup]
bucket = "deploy-baba-backups-<account-id>"
prefix = "backups/"
retain_versions = 7
schedule = "rate(1 day)"
```

---

## W-INFR.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-INFR.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-INFR.5 Test Strategy

- Coverage floor: **75%**
- Serde round-trip: `StackConfig` → TOML string → `StackConfig`
- Default values: verify `wal_mode`, `prefix`, `retain_versions`, `schedule` defaults
- Error cases: missing required fields, invalid `DeployMode` string

---

## W-INFR.6 Cross-References
- → ADR-002 (SQLite over PostgreSQL)
- → ADR-005 (zero-cost philosophy)
- → ADR-006 (EFS + SQLite + S3 topology)
- ← W-UI (reads stack.toml via StackConfig)
- ← W-XT (reads stack.toml for deploy mode, backup config)
- → `plans/cross-cutting/dependency-graph.md`
