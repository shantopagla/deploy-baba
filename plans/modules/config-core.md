# W-CFG: config-core
**Crate:** `crates/config-core/` | **Status:** DONE
**Coverage floor:** 90% | **Depends on:** (none) | **Depended on by:** W-CFGT, W-CFGY, W-CFGJ, W-INFR, W-UI

---

## W-CFG.1 Purpose

Core traits and types for the configuration layer. Defines the `ConfigSource` trait
that all format-specific crates implement. Zero runtime overhead — generic over
format type, monomorphized at compile time.

→ ADR-005 (zero-cost philosophy)

---

## W-CFG.2 Public API Surface

```rust
/// Implemented by config-toml, config-yaml, config-json
pub trait ConfigSource {
    type Error: std::error::Error + Send + Sync + 'static;
    fn load(input: &str) -> Result<serde_json::Value, Self::Error>;
    fn dump(value: &serde_json::Value) -> Result<String, Self::Error>;
}

/// Generic config container — holds parsed value + source metadata
pub struct Config<S: ConfigSource> {
    pub value: serde_json::Value,
    _source: PhantomData<S>,
}

/// Error types (thiserror, not anyhow)
#[derive(Debug, thiserror::Error)]
pub enum ConfigError { ... }
```

---

## W-CFG.3 Implementation Notes

- Source: extracted from `~/shanto/crates/rust-config-core`, renamed
- Added `ConfigSource` trait (was previously internal to each format crate)
- Uses `PhantomData<S>` to carry source type without runtime cost
- `serde_json::Value` as the common interchange format between layers
- All public items have rustdoc with `# Examples` section

---

## W-CFG.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-CFG.4.1 | Per-crate README.md | TODO | W-DX.3 dependency |

---

## W-CFG.5 Test Strategy

- Coverage floor: **90%**
- Unit tests: `load`/`dump` round-trip for valid inputs
- Error cases: malformed input, empty string, nested structures
- Property tests: round-trip invariant (if feasible with proptest)

---

## W-CFG.6 Cross-References
- → ADR-005 (zero-cost abstractions)
- ← W-CFGT, W-CFGY, W-CFGJ (implement ConfigSource)
- ← W-UI (consumes config-core directly)
- → `plans/cross-cutting/dependency-graph.md`
