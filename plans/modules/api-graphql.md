# W-APIG: api-graphql
**Crate:** `crates/api-graphql/` | **Status:** DONE
**Coverage floor:** 80% | **Depends on:** W-API | **Depended on by:** W-APIM

---

## W-APIG.1 Purpose

GraphQL SDL (Schema Definition Language) generation from `ApiSchema`. Stub implementation
(~250 lines) that produces valid GraphQL type definitions. Used by `api-merger` to
demonstrate multi-format spec generation.

Not used directly in `services/ui/` — the live demo only covers OpenAPI. GraphQL is
demonstrated via static examples in the crate map section.

---

## W-APIG.2 Public API Surface

```rust
pub struct GraphQlSpec;

impl ApiSpec for GraphQlSpec {
    type Error = GraphQlError;
    type Output = String;  // GraphQL SDL string
    fn generate(schema: &ApiSchema) -> Result<String, GraphQlError> { ... }
}

#[derive(Debug, thiserror::Error)]
pub enum GraphQlError { ... }
```

---

## W-APIG.3 Implementation Notes

- Source: Stub completed from `~/shanto/crates/rust-api-graphql` (~250 lines)
- Output format: GraphQL SDL — e.g. `type Query { ... }` with field types
- No external GraphQL library dependency — generates SDL string directly
- Type mapping: `FieldType::String → String`, `Integer → Int`, `Float → Float`,
  `Boolean → Boolean`, `Array(T) → [T]`, `Object → JSON` (scalar)

---

## W-APIG.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-APIG.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-APIG.5 Test Strategy

- Coverage floor: **80%**
- Unit tests: `generate()` produces valid GraphQL SDL from various `ApiSchema` inputs
- Snapshot tests for generated SDL strings (or string-contains checks)

---

## W-APIG.6 Cross-References
- → W-API (ApiSpec trait)
- ← W-APIM (merged alongside openapi/grpc)
- → `plans/cross-cutting/dependency-graph.md`
