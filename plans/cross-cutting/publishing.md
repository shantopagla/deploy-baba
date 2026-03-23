# W-PUB: Publishing Plan — crates.io

**Status:** TODO (Phase 9)
**Prerequisite:** Phase 8 (quality pass) complete

---

## W-PUB.1 Publish Order

Must follow dependency order (crates.io requires dependencies to be available first):

```
1. config-core
2. api-core
3. config-toml
4. config-yaml
5. config-json
6. api-openapi
7. api-graphql
8. api-grpc
9. infra-types
10. api-merger
```

---

## W-PUB.2 Pre-Publish Checklist

For each crate:
- [ ] `publish = true` in `Cargo.toml` (not set to `false`)
- [ ] `version = "0.1.0"` consistent across workspace
- [ ] `license = "MIT OR Apache-2.0"`
- [ ] `description` field set (required by crates.io)
- [ ] `repository = "https://github.com/shantopagla/deploy-baba"`
- [ ] `readme = "README.md"` (per-crate README exists)
- [ ] All public items have rustdoc comments
- [ ] No `path` dependencies without `version` field

**Not published:**
- `services/ui/` — binary crate, `publish = false`
- `xtask/` — internal tooling, `publish = false`

---

## W-PUB.3 Dry Run

```bash
just publish-dry
# → cargo xtask publish dry-run
# Runs `cargo publish --dry-run` for all 10 crates in order
# Checks packaging, validates Cargo.toml fields, no actual upload
```

---

## W-PUB.4 Release Steps

```bash
# 1. Final quality gate
just quality

# 2. Verify dry run passes
just publish-dry

# 3. Tag the release
git tag v0.1.0
git push origin v0.1.0

# 4. Publish (requires CARGO_REGISTRY_TOKEN env var)
just publish
# → cargo xtask publish release
# Publishes in dependency order with 30s sleep between each
# (crates.io index propagation delay)
```

---

## W-PUB.5 Post-Publish

- [ ] Verify all 10 crates appear on crates.io
- [ ] Add crates.io badges to root `README.md`
- [ ] Add crates.io badges to per-crate README files
- [ ] Add live Function URL to root README as "See it live" link
- [ ] GitHub release with changelog

---

## Cross-References
- → `plans/INDEX.md` — Phase 9 status
- → `plans/cross-cutting/dependency-graph.md` — publish order
- → `plans/cross-cutting/quality-gates.md` — must pass before publish
- → `plans/modules/dx-justfile.md` — W-DX.2 command reference (publish recipes)
