# Quality Gates — deploy-baba

**Command:** `just quality` → `cargo xtask quality all`
**CI:** `.github/workflows/ci.yml` — runs on every PR

---

## Coverage Floors

All 10 library crates must meet these minimums (enforced by `cargo xtask coverage`):

```
config-core:    90%
api-core:       90%
config-toml:    85%
config-yaml:    85%
config-json:    85%
api-openapi:    80%
api-graphql:    80%
api-grpc:       80%
api-merger:     80%
infra-types:    75%
```

**Tool:** `cargo-llvm-cov` (`cargo install cargo-llvm-cov`)
**Binary excluded:** `services/ui/` and `xtask/` are excluded from coverage floors
(binary crates, tested via integration).

---

## `just quality` Pipeline

```
just quality
  └─► cargo xtask quality all
        ├─ cargo xtask build format --check    (formatting)
        ├─ cargo xtask build lint           (clippy, warnings = errors)
        ├─ cargo xtask test unit            (unit tests, no external deps)
        ├─ cargo xtask coverage check       (per-crate floors)
        └─ cargo audit                      (dependency security audit)
```

Must pass completely before any deploy:
```
just deploy PROFILE  →  just quality && push-image && update Lambda
```

---

## CI Gate (`.github/workflows/ci.yml`)

Triggered on: `push` to `main`, all pull requests.

```yaml
jobs:
  check:
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test --workspace
    - cargo doc --no-deps --workspace  (doc-check)
    - cargo audit
```

Coverage floors are checked locally with `just quality` but not in CI
(avoids slow coverage instrumentation on every PR). Coverage is a pre-deploy gate.

---

## `cargo audit` Policy

- Zero known vulnerabilities in direct dependencies
- `cargo audit` is run as part of `just quality` and as a standalone `just audit`
- Unmaintained crate warnings do not fail the gate (only vulnerabilities do)

---

## Doc Coverage

All public items in library crates must have rustdoc documentation.
Enforced by `cargo doc --no-deps --workspace` (warns on missing docs).
The CI `doc-check` step fails on warnings via `RUSTDOCFLAGS="-D warnings"`.

---

## Known Gaps (Phase 0 fixes — see W-QA)

These 5 deviations were found and fixed (Phase 0 complete):

| ID | Gap | Location | Fix |
|----|-----|----------|-----|
| W-QA.0.1 | `just test-crate` passes `--crate` flag but clap expects `crate` subcommand | `justfile:36` | `cargo xtask test crate {{CRATE}}` — FIXED |
| W-QA.0.2 | `cargo audit` step is missing from `quality.rs` | `xtask/src/quality.rs:54` | Add step 5 — FIXED |
| W-QA.0.3 | Quality gate uses global 80% threshold instead of per-crate floors | `xtask/src/quality.rs:51` | Switch to `CoverageAction::Floors` — FIXED |
| W-QA.0.4 | `just quality` calls `quality gate` but subcommand is `all` | `justfile:48` | `cargo xtask quality all` — FIXED |
| W-QA.0.5 | `just fmt` calls `build fmt` but subcommand is `format` | `justfile:16` | `cargo xtask build format` — FIXED |

Full checklist: → `plans/cross-cutting/integration-tests.md`

## Cross-References
- → `plans/modules/xtask.md` — W-XT quality/coverage implementation
- → `plans/modules/dx-justfile.md` — W-DX justfile recipe wiring
- → `plans/cross-cutting/dependency-graph.md` — crate list for coverage
- → `plans/cross-cutting/integration-tests.md` — W-QA full test infrastructure plan
