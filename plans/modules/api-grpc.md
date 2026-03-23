# W-APIR: api-grpc
**Crate:** `crates/api-grpc/` | **Status:** DONE
**Coverage floor:** 80% | **Depends on:** W-API | **Depended on by:** W-APIM

---

## W-APIR.1 Purpose

Protocol Buffer IDL (`.proto`) generation from `ApiSchema`. Stub implementation
(~250 lines) that produces valid proto3 message and service definitions. Used by
`api-merger` to demonstrate multi-format spec generation.

Not used directly in `services/ui/` — demonstrated via static examples.

---

## W-APIR.2 Public API Surface

```rust
pub struct GrpcSpec;

impl ApiSpec for GrpcSpec {
    type Error = GrpcError;
    type Output = String;  // proto3 IDL string
    fn generate(schema: &ApiSchema) -> Result<String, GrpcError> { ... }
}

#[derive(Debug, thiserror::Error)]
pub enum GrpcError { ... }
```

---

## W-APIR.3 Implementation Notes

- Source: Stub completed from `~/shanto/crates/rust-api-grpc` (~250 lines)
- Output format: proto3 syntax — `syntax = "proto3"; message Foo { ... }`
- No external protobuf library dependency — generates IDL string directly
- Type mapping: `FieldType::String → string`, `Integer → int32`, `Float → float`,
  `Boolean → bool`, `Array(T) → repeated T`, `Object → bytes` (opaque)
- Generates both message definition and a simple `service` with CRUD RPCs

---

## W-APIR.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-APIR.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-APIR.5 Test Strategy

- Coverage floor: **80%**
- Unit tests: `generate()` produces valid proto3 from various `ApiSchema` inputs
- String-contains checks for `syntax = "proto3"`, message name, field names

---

## W-APIR.6 Cross-References
- → W-API (ApiSpec trait)
- ← W-APIM (merged alongside openapi/graphql)
- → `plans/cross-cutting/dependency-graph.md`
