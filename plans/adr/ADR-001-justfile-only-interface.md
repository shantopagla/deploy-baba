# ADR-001: justfile Is the Only Developer Interface

**Status:** Accepted
**Date:** 2026-03-10
**Affected modules:** W-DX, W-XT

---

## Context

The project uses `cargo xtask` internally for complex multi-step operations (build, deploy,
infra, database). xtask is powerful but requires knowledge of Rust tooling and obscures
the actual developer workflow behind Cargo invocations. Contributors with varying Rust
experience levels need a consistent, approachable entry point.

Several alternatives were considered:
- Expose `cargo xtask` directly in documentation
- Use a Makefile
- Use a shell script (`scripts/dev.sh`)
- Use `just` as the sole documented interface

---

## Decision

Developers who clone this repo interact **exclusively** through `just` commands.
`cargo xtask` is never mentioned in user-facing documentation — it is purely an
implementation mechanism called by the justfile.

```
Developer → just <command> → justfile recipe → cargo xtask <subcommand>
                                              → cargo <command>
                                              → aws <command>
                                              → terraform <command>
                                              → docker <command>
```

The justfile defines the complete public API surface:
- `just dev` — daily inner loop (fmt + lint + test)
- `just quality` — full gate before deploy
- `just deploy PROFILE` — end-to-end deploy
- `just infra-bootstrap PROFILE` — first-run setup
- See `plans/modules/dx-justfile.md` (W-DX) for the full command reference

---

## Consequences

**Positive:**
- Consistent UX regardless of Rust experience level
- Justfile serves as executable documentation
- Adding new automation = adding a justfile recipe (no API surface decision)
- `just --list` is self-documenting

**Negative:**
- Requires `just` to be installed (`brew install just`)
- justfile can become large if undisciplined (mitigated by clear section organization)
- xtask internals are tested indirectly — must ensure justfile recipes cover all code paths

**Implementation rule:** Any new automation added to xtask MUST have a corresponding
`just` recipe. xtask modules without justfile wiring are dead code.

---

## Cross-References
- `plans/modules/dx-justfile.md` — W-DX full command reference
- `plans/modules/xtask.md` — W-XT implementation details
