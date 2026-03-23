# ADR-002: SQLite Over PostgreSQL

**Status:** Accepted
**Date:** 2026-03-10
**Affected modules:** W-INFR, W-TF, W-XT

---

## Context

The original Baba Toolchain used PostgreSQL for persistent state. For a portfolio
deployment project targeting near-zero cost, a managed RDS instance (~$15–30/month)
is prohibitive. The application has low concurrency (a personal portfolio site) and
the data volume is minimal (crate metadata, stack configs, session state).

Alternatives considered:
- **PostgreSQL on RDS** — expensive, operational overhead, requires VPC
- **DynamoDB** — complex query model, no SQL, harder to inspect locally
- **SQLite in-memory** — no persistence across Lambda invocations
- **SQLite on EFS with S3 backup** — persistent, zero cost, simple

---

## Decision

Use SQLite on EFS (Elastic File System) as the sole database, with scheduled S3 backup.

```toml
[database]
path = "/mnt/db/deploy-baba.db"
wal_mode = true

[database.backup]
bucket = "deploy-baba-backups-<account-id>"
prefix = "backups/"
retain_versions = 7
schedule = "rate(1 day)"
```

The `infra-types` crate exposes `SqliteConfig` and `S3BackupConfig` structs.
PostgreSQL/MySQL engine concepts are dropped entirely.

Lambda mounts EFS at `/mnt/db/`. SQLite WAL mode enables concurrent reads
(multiple Lambda invocations can read simultaneously; writes serialize naturally
at low traffic). EventBridge triggers a daily backup Lambda to gzip-upload the
`.db` file to S3.

---

## Consequences

**Positive:**
- Zero database cost within AWS free tier
- No connection pooling, no migrations framework needed for simple schema
- SQLite file is trivially inspectable locally with any SQLite client
- S3 backup gives point-in-time recovery with configurable retention
- Eliminates VPC complexity for RDS (though EFS still requires VPC for Lambda)

**Negative:**
- Write throughput is limited (acceptable for a portfolio site, not for high-traffic apps)
- EFS adds VPC requirement (Lambda must be in VPC to mount EFS)
- EFS has a 5GB free tier only for the first 12 months; after that ~$0.001/month
- Recovery requires S3 restore operation (`just db-restore`)

**Scope constraint:** This decision is appropriate for deploy-baba's portfolio use case.
The `infra-types` crate documents this limitation clearly; library users can bring
their own database integration.

---

## Cross-References
- `plans/modules/infra-types.md` — W-INFR implementation
- `plans/cross-cutting/aws-architecture.md` — EFS mount topology
- `plans/modules/xtask.md` — W-XT.4 database backup/restore implementation
- `plans/adr/ADR-006-efs-sqlite-s3-backup.md` — EFS/S3 topology detail
