# W-CFGJ: config-json
**Crate:** `crates/config-json/` | **Status:** DONE
**Coverage floor:** 85% | **Depends on:** W-CFG | **Depended on by:** W-UI

---

## W-CFGJ.1 Purpose

JSON format implementation of `ConfigSource`. Completed from stub (~200 lines).
Supports JSON parsing for the `/api/demo/config/parse` endpoint.

---

## W-CFGJ.2 Public API Surface

```rust
pub struct JsonSource;

impl ConfigSource for JsonSource {
    type Error = JsonError;
    fn load(input: &str) -> Result<serde_json::Value, JsonError> { ... }
    fn dump(value: &serde_json::Value) -> Result<String, JsonError> { ... }
}

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
```

---

## W-CFGJ.3 Implementation Notes

- Source: Stub completed from `~/shanto/crates/rust-config-json`
- Dependency: `serde_json = "1"` via workspace (already present)
- Trivial implementation — JSON is already `serde_json::Value`
- `load`: `serde_json::from_str` → `serde_json::Value`
- `dump`: `serde_json::to_string_pretty`

---

## W-CFGJ.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-CFGJ.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-CFGJ.5 Test Strategy

- Coverage floor: **85%**
- Round-trip tests: JSON string → Value → JSON string
- Edge cases: null, arrays, nested objects, numbers
- Compact vs pretty print output

---

## W-CFGJ.6 Cross-References
- → W-CFG (ConfigSource trait)
- ← W-UI (demo endpoint)
- → `plans/cross-cutting/dependency-graph.md`
