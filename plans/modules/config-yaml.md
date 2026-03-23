# W-CFGY: config-yaml
**Crate:** `crates/config-yaml/` | **Status:** DONE
**Coverage floor:** 85% | **Depends on:** W-CFG | **Depended on by:** W-UI

---

## W-CFGY.1 Purpose

YAML format implementation of `ConfigSource`. Completed from stub (~200 lines).
Supports YAML parsing for the `/api/demo/config/parse` endpoint.

---

## W-CFGY.2 Public API Surface

```rust
pub struct YamlSource;

impl ConfigSource for YamlSource {
    type Error = YamlError;
    fn load(input: &str) -> Result<serde_json::Value, YamlError> { ... }
    fn dump(value: &serde_json::Value) -> Result<String, YamlError> { ... }
}

#[derive(Debug, thiserror::Error)]
pub enum YamlError {
    #[error("YAML parse error: {0}")]
    Parse(#[from] serde_yaml::Error),
}
```

---

## W-CFGY.3 Implementation Notes

- Source: Stub completed from `~/shanto/crates/rust-config-yaml`
- Dependency: `serde_yaml = "0.9"` via workspace
- Note: `serde_yaml` directly serializes/deserializes `serde_json::Value` via `serde`
- Simpler implementation than TOML (no intermediate type needed)

---

## W-CFGY.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-CFGY.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-CFGY.5 Test Strategy

- Coverage floor: **85%**
- Round-trip tests: YAML → Value → YAML
- Edge cases: multiline strings, YAML anchors, null values
- Integration: used live in `services/ui/` via `/api/demo/config/parse`

---

## W-CFGY.6 Cross-References
- → W-CFG (ConfigSource trait)
- ← W-UI (demo endpoint)
- → `plans/cross-cutting/dependency-graph.md`
