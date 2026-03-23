# deploy-baba — Plan Index
**GitHub:** `shantopagla/deploy-baba` | **Last updated:** 2026-03-18
**Source repo:** `~/shanto` (Baba Toolchain, ~85K LOC) | **Status:** ~85% complete

See `plans/CONVENTIONS.md` for notation system, domain codes, and file naming rules.

---

## Module Status Table

| Module | Domain | Path | Status | Key Remaining Work |
|--------|--------|------|--------|--------------------|
| config-core | W-CFG | `crates/config-core/` | DONE | Per-crate README (W-DX.3) |
| config-toml | W-CFGT | `crates/config-toml/` | DONE | Per-crate README |
| config-yaml | W-CFGY | `crates/config-yaml/` | DONE | Per-crate README |
| config-json | W-CFGJ | `crates/config-json/` | DONE | Per-crate README |
| api-core | W-API | `crates/api-core/` | DONE | Per-crate README |
| api-openapi | W-APIO | `crates/api-openapi/` | DONE | Per-crate README |
| api-graphql | W-APIG | `crates/api-graphql/` | DONE | Per-crate README |
| api-grpc | W-APIR | `crates/api-grpc/` | DONE | Per-crate README |
| api-merger | W-APIM | `crates/api-merger/` | DONE | Per-crate README |
| infra-types | W-INFR | `crates/infra-types/` | DONE | Per-crate README |
| ui-service | W-UI | `services/ui/` | DONE | utoipa-rapidoc wiring (using inline HTML) |
| xtask | W-XT | `xtask/` | WIP | CLI naming mismatch (`fmt` vs `Format`), `EnvironmentInterpolator` unused |
| terraform | W-TF | `infra/` | DONE | Fix `is_enabled`→`state`, add `filter {}` to lifecycle rules |
| dx-justfile | W-DX | `justfile`, `docs/`, `examples/` | WIP | Per-crate READMEs, integration tests |

---

## Remaining Work — Priority Order

### P1 — Must Fix (blocking clean CI)
1. ~~**W-XT.4.1**~~ — CLI naming: 3 justfile mismatches fixed (`fmt`→`format`, `--crate`→`crate` subcommand, `gate`→`all`) — **RESOLVED**
2. **W-TF.4.1** — `infra/eventbridge.tf`: replace `is_enabled = true` with `state = "ENABLED"`
3. **W-TF.4.2** — `infra/s3.tf`: add `filter {}` to each lifecycle rule
4. **W-XT.4.2** — Remove or wire up `EnvironmentInterpolator` (dead code)

### P2 — Quality Gate
5. **W-DX.3** — Per-crate README files (10 library crates)
6. **W-DX.4** — 4 standalone examples under `examples/`
7. **W-DX.5** — Integration tests for `just dev` pipeline
8. **W-XT.4.3** — Implement `just infra-bootstrap` (xtask bootstrap.rs) — creates S3 + DynamoDB + SSM
9. **W-QA** — Integration tests & test infrastructure (`plans/cross-cutting/integration-tests.md`) — 5 Phase-0 fixes done, add ~39 tests across phases 1–6

### P3 — Polish & Publish
9. **W-PUB.1** — `just publish-dry` passes for all 10 library crates
10. **W-PUB.2** — Tag `v0.1.0` + `just publish`
11. **W-UI.4.1** — Wire utoipa-rapidoc properly (currently using inline HTML)

---

## ADR Index

| ID | Title | Affected Modules |
|----|-------|-----------------|
| ADR-001 | justfile Is the Only Interface | W-DX, W-XT |
| ADR-002 | SQLite Over PostgreSQL | W-INFR, W-TF, W-XT |
| ADR-003 | Lambda Function URL (No API Gateway) | W-TF, W-UI |
| ADR-004 | Dual-Mode Entry Point | W-UI |
| ADR-005 | Zero-Cost Philosophy | W-CFG, W-API, W-INFR |
| ADR-006 | EFS + SQLite + S3 Backup | W-INFR, W-TF, W-XT |

---

## Drift Log Index

| ID | Date | Topic | Items |
|----|------|-------|-------|
| DRL-2026-03-18-terraform | 2026-03-18 | Terraform first-run gaps | 6 entries |
| DRL-2026-03-18-xtask | 2026-03-18 | xtask/justfile gaps | 7 entries |

---

## Dependency Graph Summary

```
config-core  ←── config-toml, config-yaml, config-json, infra-types (optional), services/ui
api-core     ←── api-openapi, api-graphql, api-grpc, api-merger, services/ui
api-openapi  ←── api-merger, services/ui
api-graphql  ←── api-merger
api-grpc     ←── api-merger
```

Full dependency order: `plans/cross-cutting/dependency-graph.md`

---

## Repository Structure

```
shantopagla/deploy-baba/
├── Cargo.toml              # Workspace (resolver = "2")
├── justfile                # THE developer interface
├── stack.toml              # Example stack definition
├── crates/                 # 10 library crates (all publishable)
│   ├── config-core/
│   ├── config-toml/
│   ├── config-yaml/
│   ├── config-json/
│   ├── api-core/
│   ├── api-openapi/
│   ├── api-graphql/
│   ├── api-grpc/
│   ├── api-merger/
│   └── infra-types/
├── services/ui/            # Portfolio site + deployed Lambda binary
├── xtask/                  # Internal tooling (called by justfile)
├── infra/                  # Terraform (Lambda + EFS + S3 + EventBridge)
├── examples/               # 4 standalone examples
├── docs/                   # aws-setup.md, architecture.md, etc.
└── plans/                  # This plan system (replaces PLAN.md monolith)
    ├── INDEX.md            # ← you are here
    ├── CONVENTIONS.md
    ├── adr/
    ├── modules/
    ├── cross-cutting/
    └── drift/
```

---

## Build Phase Progress

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Scaffold (workspace, justfile, stubs) | DONE |
| 2 | Extract & clean library crates | DONE |
| 3 | Complete library stubs | DONE |
| 4 | xtask modules | WIP (CLI naming fix needed) |
| 5 | UI service | DONE |
| 6 | Terraform + end-to-end deploy | WIP (TF warnings pending) |
| 7 | Examples + docs | TODO |
| 8 | Quality pass | TODO |
| 9 | Publish | TODO |
