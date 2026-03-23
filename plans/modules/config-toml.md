# W-CFGT: config-toml
**Crate:** `crates/config-toml/` | **Status:** DONE
**Coverage floor:** 85% | **Depends on:** W-CFG | **Depended on by:** W-UI

---

## W-CFGT.1 Purpose

TOML format implementation of `ConfigSource`. Parses TOML strings into
`serde_json::Value` and serializes back. Used by `services/ui/` to parse
`stack.toml` and by the `/api/demo/config/parse` endpoint.

---

## W-CFGT.2 Public API Surface

```rust
/// Implements ConfigSource for TOML format
pub struct TomlSource;

impl ConfigSource for TomlSource {
    type Error = TomlError;
    fn load(input: &str) -> Result<serde_json::Value, TomlError> { ... }
    fn dump(value: &serde_json::Value) -> Result<String, TomlError> { ... }
}

#[derive(Debug, thiserror::Error)]
pub enum TomlError {
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("TOML serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}
```

---

## W-CFGT.3 Implementation Notes

- Source: `~/shanto/crates/rust-config-toml`, renamed + minor cleanup
- Dependency: `toml = "0.8"` via workspace
- Conversion path: TOML string → `toml::Value` → `serde_json::Value` (via `serde`)
- The `dump` direction converts `serde_json::Value` → `toml::Value` → TOML string

---

## W-CFGT.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-CFGT.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-CFGT.5 Test Strategy

- Coverage floor: **85%**
- Round-trip tests: TOML → Value → TOML
- Error tests: invalid TOML syntax
- Integration: used live in `services/ui/` via `/api/demo/config/parse`

---

## W-CFGT.6 Cross-References
- → W-CFG (ConfigSource trait)
- ← W-UI (parses stack.toml, demo endpoint)
- → `plans/cross-cutting/dependency-graph.md`
