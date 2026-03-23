# W-APIM: api-merger
**Crate:** `crates/api-merger/` | **Status:** DONE
**Coverage floor:** 80% | **Depends on:** W-API, W-APIO, W-APIG, W-APIR | **Depended on by:** W-UI

---

## W-APIM.1 Purpose

Orchestrates multi-format API spec generation. Given an `ApiSchema`, generates
OpenAPI + GraphQL SDL + proto3 simultaneously and returns all three. Also supports
merge strategies for combining multiple `ApiSchema` inputs into one.

Used by `services/ui/` for the `/api/demo/spec/generate` endpoint.

---

## W-APIM.2 Public API Surface

```rust
pub struct SpecMerger;

/// Generates all three formats from a single schema
pub struct MergedSpec {
    pub openapi: serde_json::Value,
    pub graphql: String,
    pub grpc: String,
}

impl SpecMerger {
    pub fn generate_all(schema: &ApiSchema) -> Result<MergedSpec, MergerError> { ... }
    pub fn merge(schemas: &[ApiSchema], strategy: MergeStrategy) -> Result<ApiSchema, MergerError> { ... }
}

pub enum MergeStrategy {
    Union,       // include all fields from all schemas
    Intersection, // only fields present in all schemas
    FirstWins,   // first schema takes precedence on conflict
}

#[derive(Debug, thiserror::Error)]
pub enum MergerError { ... }
```

---

## W-APIM.3 Implementation Notes

- Source: `~/shanto/crates/rust-api-merger`, extra strategies added, internals renamed
- `generate_all` calls `OpenApiSpec::generate`, `GraphQlSpec::generate`, `GrpcSpec::generate`
- `merge` strategies implemented for `Union` and `FirstWins`; `Intersection` is a stretch goal
- Extra strategies added beyond original: `FirstWins` variant

---

## W-APIM.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-APIM.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-APIM.5 Test Strategy

- Coverage floor: **80%**
- Unit tests: `generate_all` produces valid output for all three formats
- Merge strategy tests: Union includes all fields, FirstWins respects order
- Integration: used live at `/api/demo/spec/generate`

---

## W-APIM.6 Cross-References
- → W-API, W-APIO, W-APIG, W-APIR (all four spec implementations)
- ← W-UI (generate endpoint)
- → `plans/cross-cutting/dependency-graph.md`
