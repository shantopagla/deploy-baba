# DRL-2026-03-25-opentofu — OpenTofu Migration Observations

**Date:** 2026-03-25
**Topic:** Pre-migration audit of Terraform HCL vs plan status

---

## Summary

Before writing the OpenTofu migration plan (W-OTF), the actual HCL files in `infra/` were audited
against the open work items listed in `plans/modules/terraform.md` (W-TF.4). Two items listed as
OPEN in the plan are already fixed in the source code.

---

## Drift Entries

### DRL-OTF-1 — W-TF.4.1 Already Fixed

**Plan says:** `infra/eventbridge.tf` line 6: replace `is_enabled = true` with `state = "ENABLED"` — OPEN

**Reality:** `infra/eventbridge.tf` already uses `state = "ENABLED"` at line 6. The `is_enabled`
deprecation was fixed at some point but the plan item was never closed.

**Action:** Mark W-TF.4.1 as RESOLVED in INDEX.md and terraform.md.

---

### DRL-OTF-2 — W-TF.4.2 Already Fixed

**Plan says:** `infra/s3.tf`: add `filter {}` to each lifecycle rule — OPEN

**Reality:** `infra/s3.tf` already contains `filter {}` (empty filter block) inside the
`aws_s3_bucket_lifecycle_configuration.backups` rule block at line 48. Fix was applied but
plan item was not updated.

**Action:** Mark W-TF.4.2 as RESOLVED in INDEX.md and terraform.md.

---

### DRL-OTF-3 — `main.tf` Declares `required_version = ">= 1.0"`

**Observation:** The `required_version` in `infra/main.tf` is `>= 1.0`, which technically matches
any Terraform/OpenTofu version but does not signal the binary switch.

**Action:** W-OTF.4.3 updates this to `>= 1.6` (OpenTofu's first stable release), providing a
clear floor that excludes HashiCorp Terraform 1.x and documents the intended binary.

---

### DRL-OTF-4 — `ManagedBy = "Terraform"` Tag in `locals`

**Observation:** `infra/main.tf` `locals.common_tags` contains `ManagedBy = "Terraform"`. After
migration, all existing resources tagged with this value will show "Terraform" in AWS Console.

**Action:** W-OTF.4.3 changes this to `"OpenTofu"`. On next `tofu apply`, AWS will update all
tagged resources (tag-only change, no resource recreation).

---

### DRL-OTF-5 — xtask Uses `anyhow::anyhow!` for Error Construction

**Observation:** `xtask/src/infra/terraform.rs` uses `anyhow::anyhow!(...)` for error wrapping.
The project CLAUDE.md says to use `thiserror` in library crates but xtask is internal tooling —
`anyhow` is appropriate here (consistent with existing usage throughout xtask).

**Action:** No change needed. Keep `anyhow` in xtask.

---

### DRL-OTF-6 — No `.terraform.lock.hcl` in Git

**Observation:** `infra/.terraform.lock.hcl` is not committed (not found in the repo tree).
This means `tofu init` will create a fresh lock file on first run.

**Action:** After W-OTF.4.7 (smoke test), commit the generated `infra/.terraform.lock.hcl`
to lock provider versions. Add a note in `plans/cross-cutting/aws-setup-spec.md`.

---

## Plan Status Updates Required

| Item | Old Status | Correct Status | File to Update |
|------|-----------|----------------|----------------|
| W-TF.4.1 | OPEN | RESOLVED | INDEX.md, terraform.md |
| W-TF.4.2 | OPEN | RESOLVED | INDEX.md, terraform.md |
| W-TF (module) | DONE | SUPERSEDED by W-OTF | INDEX.md, terraform.md |
