# W-APIO: api-openapi
**Crate:** `crates/api-openapi/` | **Status:** DONE
**Coverage floor:** 80% | **Depends on:** W-API | **Depended on by:** W-APIM, W-UI

---

## W-APIO.1 Purpose

OpenAPI 3.0 spec generation from `ApiSchema`. The primary format used by `services/ui/`
for both the live OpenAPI spec (`GET /api/openapi.json`) and the
`POST /api/demo/spec/generate` endpoint.

Also integrates with `utoipa` for the `#[derive(OpenApi)]` macro used in the UI service.

---

## W-APIO.2 Public API Surface

```rust
pub struct OpenApiSpec;

impl ApiSpec for OpenApiSpec {
    type Error = OpenApiError;
    type Output = serde_json::Value;  // OpenAPI 3.0 JSON
    fn generate(schema: &ApiSchema) -> Result<serde_json::Value, OpenApiError> { ... }
}

/// Serialize a utoipa-generated OpenAPI spec to JSON
pub fn spec_to_json(spec: &utoipa::openapi::OpenApi) -> Result<String, OpenApiError> { ... }

#[derive(Debug, thiserror::Error)]
pub enum OpenApiError { ... }
```

---

## W-APIO.3 Implementation Notes

- Source: `~/shanto/crates/rust-api-openapi`, extracted and polished
- Integration with `utoipa = { version = "4", features = ["axum_extras"] }`
- `generate()` produces a minimal valid OpenAPI 3.0 JSON object from `ApiSchema`
- The live spec at `/api/openapi.json` is assembled in `services/ui/openapi.rs` using
  `#[derive(OpenApi)]` — separate from the `generate()` path

---

## W-APIO.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-APIO.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-APIO.5 Test Strategy

- Coverage floor: **80%**
- Unit tests: `generate()` produces valid OpenAPI 3.0 structure
- Validate output against OpenAPI schema (or spot-check required fields)
- Integration: used live at `/api/demo/spec/generate`

---

## W-APIO.6 Cross-References
- → W-API (ApiSpec trait)
- ← W-APIM (merged alongside graphql/grpc)
- ← W-UI (live demo endpoint + OpenAPI spec assembly)
- → `plans/cross-cutting/dependency-graph.md`
