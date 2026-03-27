# CLAUDE.md — Portfolio Project Instructions

This file provides guidance to Claude Code when working in this repository.
It includes global instructions (inlined from `~/CLAUDE.md`) plus project-specific context.

---

## About Me

- **Name:** shantopagla
- **Email:** it@shantopagla.com
- **GitHub:** shantopagla
- **Primary Language:** Rust (also uses TypeScript/Node.js, Python)
- **Cloud Platform:** AWS (us-east-1)
- **OS:** macOS
- **Shell:** zsh
- **Package Managers:** cargo, npm (via nvm), pip (via pyenv), brew

## Development Environment

- **Rust:** Managed via rustup, cargo in `~/.cargo/bin`
- **Node.js:** Managed via nvm (`~/.nvm`)
- **Python:** Managed via pyenv (`~/.pyenv`)
- **Editor:** VS Code / Cursor
- **Git:** Default branch is `main`, uses git-lfs

## Coding Preferences

- Write clean, idiomatic code for the target language
- Prefer Rust's trait-based composition and zero-cost abstractions
- Use `thiserror` for error handling in Rust, not `anyhow` in library crates
- Follow existing project conventions and patterns
- Always run `cargo fmt` and `cargo clippy` before committing Rust code
- Use `just` commands when available in a project
- Prefer async/await patterns with tokio

## Git Conventions

- Commit messages: concise, imperative mood ("Add feature" not "Added feature")
- Default branch: `main`
- Use conventional commit prefixes when appropriate: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`

## AWS & Infrastructure

- Default region: `us-east-1`
- Uses AWS SSO for authentication
- Infrastructure managed via OpenTofu (generated from Rust types)
- Deployment philosophy: zero-cost first, scale up only when needed

## Security Notes

- Never commit credentials, API keys, or secrets to git
- Use environment variables or AWS Secrets Manager for sensitive values
- Check `.gitignore` before staging files

---

## Project: deploy-baba

Zero-cost Rust portfolio and deployment automation platform hosted on AWS Lambda.

### Workspace Structure

```
portfolio/
├── crates/           # 10 library crates (pure Rust, no binaries)
├── services/ui/      # Lambda binary (the deployed service)
├── xtask/            # Internal CLI — do NOT call directly
├── examples/         # 4 example binaries
├── infra/            # OpenTofu (Lambda + EFS + S3 + EventBridge + CloudFront)
├── plans/            # Modular plan system (see plans/INDEX.md)
├── stack.toml        # Local-only config (copy from stack.example.toml)
└── justfile          # The only interface — use `just` commands
```

### Task Runner — `just` is the only interface (ADR-001)

Never call `cargo xtask` directly. All commands go through `just`.

Key commands:
- `just dev` — inner development loop
- `just quality` — full quality gate (fmt + clippy + test)
- `just deploy PROFILE` — quality gate → lambda-build → infra apply
- `just ui` — run local UI server
- `just lambda-build` — build Lambda binary (uses cross for aarch64)
- `just infra-plan` / `just infra-apply` — OpenTofu plan/apply

### Architecture Decisions

- **ADR-001:** justfile-only interface — xtask is internal plumbing, never invoked directly
- **ADR-002:** SQLite on EFS + S3 backup — no PostgreSQL, no RDS
- **ADR-003:** Lambda Function URL — no API Gateway
- **ADR-004:** Dual-mode entry point — runtime env var detection (local vs Lambda)

### Stack Config (`stack.toml`)

Local-only config file — not committed to git. Contains:
- Project metadata, deploy mode, database path
- Observability settings, AWS profile

Copy `stack.example.toml` to `stack.toml` to get started. No external dependencies or remote service URLs.

### Plan System

Entry point: `plans/INDEX.md` — lists all modules, ADRs, cross-cutting concerns, and drift logs.

Structure under `plans/`:
- `modules/` — 14 per-component plans
- `adr/` — architecture decision records (ADR-001 through ADR-006)
- `cross-cutting/` — 5 shared concern files
- `drift/` — drift logs (format: `DRL-YYYY-MM-DD`)

Current status: ~85% complete. Remaining work listed in P1–P3 sections of `plans/INDEX.md`.

### Cross-Session Memory

Claude's auto-memory for this project: `~/.claude/projects/-Users-shantopagla-portfolio/memory/`
