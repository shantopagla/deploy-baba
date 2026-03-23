# ADR-004: Dual-Mode Entry Point

**Status:** Accepted
**Date:** 2026-03-10
**Affected modules:** W-UI

---

## Context

The `services/ui/` binary needs to run in two environments:
1. **AWS Lambda** — receives HTTP events via `lambda_http` adapter
2. **Local dev** — plain TCP server via `axum::serve`

Common approaches for handling this:
- Feature flags (`--features lambda` vs `--features local`)
- Separate binaries (`main_lambda.rs`, `main_local.rs`)
- Runtime environment detection
- Conditional compilation with `cfg`

---

## Decision

Use runtime environment detection via the `AWS_LAMBDA_FUNCTION_NAME` environment variable.
AWS sets this variable automatically on all Lambda executions. Local dev never sets it.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().without_time().init();
    let app = router::build();

    if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        lambda_http::run(app).await?;
    } else {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        println!("→ http://localhost:3000");
        axum::serve(listener, app).await?;
    }
    Ok(())
}
```

No feature flags. Single binary. `cargo-lambda build --release` produces the Lambda zip.
`just ui` / `cargo run` runs locally.

---

## Consequences

**Positive:**
- Single binary — no build variants to manage
- Clean, testable code — no conditional compilation
- `router::build()` is identical in both modes (same Axum router)
- Local dev experience identical to Lambda behavior

**Negative:**
- `lambda_http` dependency is always compiled in (adds ~100KB to binary)
- If `AWS_LAMBDA_FUNCTION_NAME` is accidentally set locally, runs in Lambda mode
  (mitigated by clear documentation; this env var is highly unlikely to be set locally)

**Binary size optimization:** Release profile uses `lto = true`, `opt-level = "z"`,
`codegen-units = 1`, `strip = true` to keep the Lambda zip under 10MB.

---

## Cross-References
- `plans/modules/ui-service.md` — W-UI full implementation spec
- `plans/cross-cutting/aws-architecture.md` — Lambda invocation topology
- `plans/adr/ADR-003-lambda-function-url.md` — HTTPS endpoint decision
