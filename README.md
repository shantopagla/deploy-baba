# deploy-baba

**Zero-cost Rust abstractions for deployment automation.**

A composable crate ecosystem for configuration parsing, API specification
generation, and infrastructure type definitions — built on trait-based
composition with monomorphization, not dynamic dispatch.

```
┌─────────────────────────────────────────────────────┐
│              Config Layer                            │
│  config-core → config-toml / config-yaml / config-json │
├─────────────────────────────────────────────────────┤
│             API Spec Layer                           │
│  api-core → api-openapi / api-graphql / api-grpc    │
│                    → api-merger                      │
├─────────────────────────────────────────────────────┤
│           Infrastructure Layer                       │
│  infra-types (Stack, Service, SQLite, Network)       │
└─────────────────────────────────────────────────────┘
```

## Quick Start

```rust
use config_toml::TomlParser;
use config_core::ConfigParser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyConfig {
    name: String,
    version: String,
}

// Parse from TOML — swap TomlParser for YamlParser or JsonParser
// with zero code changes. Same trait, same generic bounds.
let config: MyConfig = TomlParser::parse(r#"
    name = "my-app"
    version = "1.0.0"
"#)?;
```

## Crate Map

| Crate | Purpose |
|-------|---------|
| `config-core` | Universal traits: `ConfigParser<T>`, `ConfigValidator<T>` |
| `config-toml` | TOML implementation |
| `config-yaml` | YAML implementation |
| `config-json` | JSON implementation |
| `api-core` | Universal traits: `ApiSpecGenerator` |
| `api-openapi` | OpenAPI 3.0 generator (via utoipa) |
| `api-graphql` | GraphQL SDL generator |
| `api-grpc` | Protocol Buffers / gRPC generator |
| `api-merger` | Multi-format spec merging with conflict resolution |
| `infra-types` | Cloud-agnostic Stack, Service, Network, SQLite + S3 types |

## Development

All commands go through the [justfile](justfile). Run `just` to see everything.

```bash
just dev          # format + lint + test
just ui           # run the portfolio site locally at localhost:3000
just quality      # full quality gate
just docs         # build and open rustdoc
```

## Examples

Runnable examples are in [`examples/`](examples/). Each is a standalone package
in the workspace — use `just example <name>` to run them.

```bash
just example 01_multi_format_config   # Parse the same config as TOML, YAML, and JSON
just example 02_api_spec_generation   # Generate OpenAPI, GraphQL, and Protobuf specs
just example 03_spec_merger           # Merge multiple specs with conflict resolution
just example 04_infra_types           # Build and serialize a Stack definition
```

## Deploy to AWS (Optional)

The portfolio site runs on AWS Lambda with a Function URL — near-zero cost.

```bash
just aws-check deploy-baba       # validate your AWS profile
just infra-bootstrap deploy-baba  # first-time setup
just infra-apply deploy-baba      # provision infrastructure
just deploy deploy-baba           # build + push + update Lambda
just ui-open deploy-baba          # open the live site
```

See [docs/aws-setup.md](docs/aws-setup.md) for full setup instructions.

## Architecture

See [docs/architecture.md](docs/architecture.md) for the full crate dependency
graph and layer descriptions.

See [docs/zero-cost-philosophy.md](docs/zero-cost-philosophy.md) for why
everything uses generics over `dyn` dispatch.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
