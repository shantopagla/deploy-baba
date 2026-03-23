# Integration Tests & Test Infrastructure — deploy-baba

**Domain:** W-QA | **Status:** TODO | **Priority:** P2
**Last updated:** 2026-03-23

---

## Context

The workspace has 114 unit tests across 10 library crates but **zero integration tests** (`tests/` directories). The `cargo xtask test integration` subcommand exists but finds nothing. `services/ui` and `xtask` have zero tests. The quality gate (`quality.rs`) is missing `cargo audit` (step 5) and uses a global 80% threshold instead of the per-crate floors already implemented in `coverage.rs`. The `just test-crate` recipe passes `--crate` but clap expects the `crate` subcommand.

---

## Phase 0 — Fix Broken Test Infrastructure

**5 edits, 0 new tests — fix before anything else**

| ID | Fix | File | Detail |
|----|-----|------|--------|
| W-QA.0.1 | `just test-crate` recipe broken | `justfile:36` | Change `cargo xtask test --crate {{CRATE}}` → `cargo xtask test crate {{CRATE}}` (clap `Crate` is a subcommand with positional `name`, not a `--crate` flag) |
| W-QA.0.2 | Quality gate missing `cargo audit` | `xtask/src/quality.rs:54` | Add step 5 after coverage: `Command::new("cargo").args(["audit"]).status()`, fail on exit code != 0 |
| W-QA.0.3 | Quality gate uses global threshold not per-crate floors | `xtask/src/quality.rs:51` | Replace `CoverageAction::Check { threshold: 80 }` with `CoverageAction::Floors` (the `enforce_floors()` function at `coverage.rs:104` already has the correct floor map) |
| W-QA.0.4 | `just quality` calls `quality gate` but subcommand is `all` | `justfile:48` | Change `cargo xtask quality gate` → `cargo xtask quality all` |
| W-QA.0.5 | `just fmt` calls `build fmt` but subcommand is `format` | `justfile:16` | Change `cargo xtask build fmt` → `cargo xtask build format` |

**Verify:** `just test-crate config-core` works; `cargo xtask quality all` runs 5 steps.

- [x] W-QA.0.1 done
- [x] W-QA.0.2 done
- [x] W-QA.0.3 done
- [x] W-QA.0.4 done
- [x] W-QA.0.5 done

---

## Phase 1 — Config Crate Integration Tests

**4 new files, ~12 tests**

### `crates/config-core/tests/integration.rs`

Key scenarios:
- Cross-trait composition: `ConfigParser` + `ConfigValidator` on same type
- `ConfigMerger` precedence (override wins over base, base fills missing keys)
- `ConfigSource` round-trip (build source, parse back, assert equality)

### `crates/config-toml/tests/integration.rs`

Key scenarios:
- Parse-validate pipeline with `TomlValidatable` (valid → passes, invalid → `ValidationError`)
- File round-trip via `save_toml_config` / `load_toml_config` using `tempfile::tempdir()`
- Cross-crate trait usage: implement `ConfigParser` for a local test struct using `TomlParseable`

Dev-deps to add to `config-toml`: `tempfile = "3"`

### `crates/config-yaml/tests/integration.rs`

Key scenarios:
- Complex YAML features: multiline scalars, nested mappings, sequence-of-maps
- File round-trip (tempfile): save → reload → struct equality
- Error paths: invalid YAML string → `ParseError`; valid parse but fails `YamlValidatable` → `ValidationError`

Dev-deps to add to `config-yaml`: `tempfile = "3"`

### `crates/config-json/tests/integration.rs`

Key scenarios:
- Nested JSON parsing: 3-level deep struct deserialization
- File round-trip (tempfile): write JSON, read back, assert equality
- Struct equality against hardcoded expected value (golden-value test)

Dev-deps to add to `config-json`: `tempfile = "3"`

**Verify:** `cargo test --test '*' -p config-core -p config-toml -p config-yaml -p config-json`

- [ ] `crates/config-core/tests/integration.rs` created (~3 tests)
- [ ] `crates/config-toml/tests/integration.rs` created (~3 tests)
- [ ] `crates/config-yaml/tests/integration.rs` created (~3 tests)
- [ ] `crates/config-json/tests/integration.rs` created (~3 tests)

---

## Phase 2 — API Crate Integration Tests

**5 new files, ~12 tests**

### `crates/api-core/tests/integration.rs`

Key scenarios:
- `ApiSpecGenerator` lifecycle: construct → `generate_spec()` → assert non-empty output
- `generate_spec()` error path: misconfigured generator → `SpecGenerationError`
- Default `merge_specs` behavior: two non-overlapping specs → combined spec
- `SpecFormat` serde round-trip: `serde_json::to_string` → `from_str` → equality

### `crates/api-openapi/tests/integration.rs`

Key scenarios:
- End-to-end spec generation → serialize to JSON → validate JSON structure (has `openapi`, `info`, `paths` keys)
- Multi-spec merge (non-overlapping): two `OpenApiSpec` values → merged has all paths from both
- Multi-spec merge (conflict with `FirstWins`): overlapping path defined in both → first definition wins
- Metadata correctness: title, version survive the generate → serialize → deserialize round-trip

### `crates/api-graphql/tests/integration.rs`

Key scenarios:
- SDL generation: construct `GraphQLSchema` with types and fields → `generate_sdl()` → assert SDL string contains type definitions
- Schema merge: two schemas with disjoint types → merged schema has all types

### `crates/api-grpc/tests/integration.rs`

Key scenarios:
- Proto generation: construct `GrpcSchema` with services and messages → `generate_proto()` → assert proto string contains service and message definitions
- Proto merge: two schemas with disjoint services → merged has all services

### `crates/api-merger/tests/integration.rs`

Key scenarios:
- Cross-format merge rejection: passing `OpenApiSpec` and `GraphQLSchema` to merger → `FormatMismatch` error
- Full merge pipeline with `FirstWins` strategy: 3 OpenAPI specs → merged correctly, first wins on conflicts
- Empty merge: zero specs → `EmptyMergeError` or empty spec (depending on `MergePolicy`)

**Verify:** `cargo test --test '*' -p api-core -p api-openapi -p api-graphql -p api-grpc -p api-merger`

- [ ] `crates/api-core/tests/integration.rs` created (~4 tests)
- [ ] `crates/api-openapi/tests/integration.rs` created (~4 tests)
- [ ] `crates/api-graphql/tests/integration.rs` created (~2 tests)
- [ ] `crates/api-grpc/tests/integration.rs` created (~2 tests)
- [ ] `crates/api-merger/tests/integration.rs` created (~3 tests)

---

## Phase 3 — infra-types Integration Tests

**1 new file, ~4 tests**

**File:** `crates/infra-types/tests/integration.rs`

Scenarios:
- `stack.toml` deserialization: read `stack.toml` from repo root → `Stack` struct → assert non-empty name and at least one environment
- `Environment` display: `Environment::Production.to_string()` == `"production"` (or `"prod"` per impl)
- `Environment::is_production()`: `Production` → true, `Staging` → false
- `Provider::is_cloud()`: `Aws` → true, `Local` → false
- Error path: deserialize TOML with missing required field → `toml::de::Error`

**Verify:** `cargo test --test '*' -p infra-types`

- [ ] `crates/infra-types/tests/integration.rs` created (~4 tests)

---

## Phase 4 — services/ui Smoke Tests

**2 new files, ~5 tests**

### Prerequisite: Add `services/ui/src/lib.rs`

Currently all modules (`router`, `routes`, `openapi`, `error`) are declared in `main.rs` and are private to the binary — unreachable from `tests/`. Add:

**`services/ui/src/lib.rs`:**
```rust
pub mod router;
pub mod routes;
pub mod openapi;
pub mod error;
```

And in `main.rs`, replace the `mod` declarations with `use deploy_baba_ui::*;` (or keep them and add `#[path]` re-exports — match whatever pattern the existing crate uses).

Dev-deps to add to `services/ui/Cargo.toml`:
```toml
[dev-dependencies]
tower = { version = "0.4", features = ["util"] }
http-body-util = "0.1"
hyper = { version = "1", features = ["full"] }
```

### `services/ui/tests/smoke.rs`

Scenarios (all use `tower::ServiceExt::oneshot` against `router::build()` — no network):
- `GET /health` → 200, `Content-Type: application/json`, body has `"status"` key
- `GET /` → 200, `Content-Type: text/html`, body contains `<html`
- `GET /api/openapi.json` → 200, `Content-Type: application/json`, body is valid JSON
- `GET /nonexistent` → 404
- `GET /docs` → 200, `Content-Type: text/html`, body contains `rapidoc` or `rapi-doc`

**Verify:** `cargo test --test smoke -p deploy-baba-ui`

- [ ] `services/ui/src/lib.rs` created
- [ ] `services/ui/Cargo.toml` dev-deps updated
- [ ] `services/ui/tests/smoke.rs` created (~5 tests)

---

## Phase 5 — xtask Unit Tests (Inline)

**0 new files, ~6 inline `#[cfg(test)]` blocks**

### `xtask/src/coverage.rs`

Add `#[cfg(test)] mod tests` at the bottom:

| Test | Assertion |
|------|-----------|
| `parse_coverage_typical` | input `"... 85.30% ...covered"` → `Some(85.3)` |
| `parse_coverage_no_percentage` | input `"no numbers here"` → `None` |
| `parse_coverage_multiline` | multiline output with two percentages → parses the last match |
| `floor_map_completeness` | `COVERAGE_FLOORS` (or `build_floor_map()`) has exactly 10 entries, all expected crate names present |

### `xtask/src/deploy/lambda.rs`

Add `#[cfg(test)] mod tests` at the bottom:

| Test | Assertion |
|------|-----------|
| `zip_path_format` | `ZIP_PATH` ends with `.zip` and starts with `"infra/"` |
| `package_name` | `PACKAGE` == `"deploy-baba-ui"` |

**Verify:** `cargo test -p xtask`

- [ ] `xtask/src/coverage.rs` tests added (~4 tests)
- [ ] `xtask/src/deploy/lambda.rs` tests added (~2 tests)

---

## Phase 6 — Documentation Updates

**0 code changes**

- [ ] Update `plans/cross-cutting/quality-gates.md` to note that Phase 0 fixes bring the gate to 5 steps and switch to per-crate floors
- [ ] Confirm W-QA entry appears in `plans/INDEX.md` (added when this file was registered)

---

## Summary

| Phase | Scope | New files | Tests |
|-------|-------|-----------|-------|
| 0 | Fix infrastructure | 0 | 0 |
| 1 | Config integration | 4 | ~12 |
| 2 | API integration | 5 | ~12 |
| 3 | infra-types integration | 1 | ~4 |
| 4 | services/ui smoke | 2 (`lib.rs` + `smoke.rs`) | ~5 |
| 5 | xtask unit tests | 0 (inline) | ~6 |
| 6 | Docs update | 0 | 0 |
| **Total** | | **12** | **~39** |

---

## Cross-References

- → `plans/cross-cutting/quality-gates.md` — quality gate pipeline
- → `plans/modules/xtask.md` — W-XT.4.1 CLI naming fix (related)
- → `plans/modules/ui-service.md` — W-UI service architecture
- → `plans/drift/DRL-2026-03-18-xtask.md` — original drift log for xtask gaps
