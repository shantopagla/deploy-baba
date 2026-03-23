# W-API: api-core
**Crate:** `crates/api-core/` | **Status:** DONE
**Coverage floor:** 90% | **Depends on:** (none) | **Depended on by:** W-APIO, W-APIG, W-APIR, W-APIM, W-UI

---

## W-API.1 Purpose

Core traits and types for the API specification layer. Defines the `ApiSpec` trait
that OpenAPI, GraphQL, and gRPC implementations implement. Provides shared types
for field definitions used by the spec generator demo.

→ ADR-005 (zero-cost philosophy)

---

## W-API.2 Public API Surface

```rust
/// Implemented by api-openapi, api-graphql, api-grpc
pub trait ApiSpec {
    type Error: std::error::Error + Send + Sync + 'static;
    type Output;
    fn generate(schema: &ApiSchema) -> Result<Self::Output, Self::Error>;
}

/// Shared schema definition (used by all format implementations)
pub struct ApiSchema {
    pub title: String,
    pub version: String,
    pub fields: Vec<FieldDef>,
}

pub struct FieldDef {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub description: Option<String>,
}

pub enum FieldType { String, Integer, Float, Boolean, Array(Box<FieldType>), Object }

#[derive(Debug, thiserror::Error)]
pub enum ApiError { ... }
```

---

## W-API.3 Implementation Notes

- Source: `~/shanto/crates/rust-api-core`, internal fields removed, AsyncAPI stub added
- `AsyncAPI` is a placeholder stub — not fully implemented, documents the intent
- Uses `thiserror` for error types (no `anyhow` in library)
- `ApiSchema` is designed to be forward-compatible with all three spec formats

---

## W-API.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-API.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-API.5 Test Strategy

- Coverage floor: **90%**
- Unit tests: `ApiSchema` construction, `FieldDef` validation
- Error cases: empty schema, invalid field types
- Trait object tests (ensure `dyn ApiSpec` compiles with correct bounds)

---

## W-API.6 Cross-References
- → ADR-005 (zero-cost abstractions)
- ← W-APIO, W-APIG, W-APIR (implement ApiSpec)
- ← W-APIM (orchestrates across all three)
- ← W-UI (uses api-core types for demo endpoint)
- → `plans/cross-cutting/dependency-graph.md`
