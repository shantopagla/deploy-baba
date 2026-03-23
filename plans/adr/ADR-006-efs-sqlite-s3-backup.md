# ADR-006: EFS + SQLite + S3 Backup Topology

**Status:** Accepted
**Date:** 2026-03-10
**Affected modules:** W-INFR, W-TF, W-XT

---

## Context

The Lambda function needs persistent storage. Lambda itself is stateless — ephemeral
`/tmp` (512MB) is wiped between invocations when the execution environment is recycled.
EFS (Elastic File System) provides NFS-mounted persistent storage that Lambda can access
within a VPC.

---

## Decision

**Storage topology:**
```
Lambda ──(NFS mount)──► EFS /mnt/db/deploy-baba.db
                               │
                  EventBridge (daily) triggers backup Lambda
                               │
                              ▼
                        S3 backups/YYYY-MM-DDTHH:MM:SSZ.db.gz
                        (retain 7 versions)
```

**EFS configuration:**
- Access point scoped to `/deploy-baba` directory
- POSIX uid/gid 1000 (Lambda execution context)
- Mounted at `/mnt/db/` in Lambda

**SQLite configuration:**
- WAL mode enabled for concurrent reads
- `VACUUM INTO '/tmp/backup.db'` for clean backup copy (avoids WAL file)
- Backup: gzip the vacuum copy → upload to S3 with timestamp key

**S3 backup bucket:**
- Name: `deploy-baba-backups-<account-id>` (unique per account)
- Versioning enabled
- Lifecycle rule: retain 7 versions (`retain_versions = 7`)
- Server-side encryption: AES256

**Restore flow:**
```
just db-restore       → downloads latest .db.gz → decompresses → writes to EFS path
just db-restore-version VERSION → downloads specific version
```

---

## Consequences

**Positive:**
- EFS is persistent across Lambda invocations and container recycles
- S3 backup provides point-in-time recovery
- Daily backup is sufficient for portfolio data (low mutation rate)
- WAL mode handles the Lambda concurrency model cleanly

**Negative:**
- Lambda must be in a VPC to mount EFS (adds VPC provisioning to Terraform)
- VPC adds ~100ms cold start latency for the Lambda ENI attachment
- EFS has per-GB-month cost after the 12-month free tier (~$0.001/month at 1MB)
- First deploy requires EFS access point to be provisioned before Lambda can start

**Operational note:** The backup Lambda is a separate invocation triggered by EventBridge.
The backup does NOT run inside the UI Lambda — it is a separate function or a `just db-backup`
manual trigger via xtask.

---

## Cross-References
- `plans/adr/ADR-002-sqlite-over-postgresql.md` — the database choice decision
- `plans/modules/infra-types.md` — W-INFR SqliteConfig + S3BackupConfig structs
- `plans/modules/terraform.md` — W-TF EFS + S3 Terraform resources
- `plans/modules/xtask.md` — W-XT database backup/restore implementation
- `plans/drift/DRL-2026-03-18-terraform.md` — EFS security group cycle fix
