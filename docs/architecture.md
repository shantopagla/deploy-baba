# Architecture

deploy-baba is organized as a layered Rust workspace where each crate solves one
problem perfectly. Combined, they form a composable ecosystem for deployment
automation.

## Crate Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                    services/ui                               │
│   Portfolio site + live API demo (Axum + Lambda adapter)     │
└────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬───────┘
     │      │      │      │      │      │      │      │
     ▼      ▼      ▼      ▼      ▼      ▼      ▼      ▼
┌────────┐┌────────┐┌────────┐┌────────┐┌─────────┐┌──────────┐
│config- ││config- ││config- ││config- ││   api-  ││  infra-  │
│ core   ││ toml   ││ yaml   ││ json   ││ merger  ││  types   │
└────────┘└───┬────┘└───┬────┘└───┬────┘└────┬────┘└──────────┘
              │         │         │          │
              ▼         ▼         ▼          ▼
         ┌────────┐               ┌──────┐┌──────┐┌──────┐
         │config- │               │ api- ││ api- ││ api- │
         │ core   │               │openapi││graphql│ grpc │
         └────────┘               └──┬───┘└──┬───┘└──┬───┘
                                     │       │       │
                                     ▼       ▼       ▼
                                  ┌──────────────────────┐
                                  │      api-core        │
                                  └──────────────────────┘
```

## Layer Descriptions

### Config Layer

Format-agnostic configuration parsing through universal traits. Write one
`ConfigParser<T>` implementation per format; consumers write generic code that
works with any format.

- **config-core**: Trait definitions (`ConfigParser<T>`, `ConfigValidator<T>`)
- **config-toml**: TOML implementation
- **config-yaml**: YAML implementation
- **config-json**: JSON implementation

### API Spec Layer

Generate API specifications in multiple wire formats from a single Rust type
definition. The merger combines specs from different services.

- **api-core**: Trait definitions (`ApiSpecGenerator`)
- **api-openapi**: OpenAPI 3.0 generator (via utoipa)
- **api-graphql**: GraphQL SDL generator
- **api-grpc**: Protocol Buffer generator
- **api-merger**: Multi-format merging with conflict resolution

### Infrastructure Layer

Cloud-agnostic type definitions for deployment infrastructure. SQLite + S3
backup as the database layer.

- **infra-types**: Stack, Service, Network, SqliteConfig, ObservabilityConfig

### Services

- **ui**: The portfolio landing page and live API demo. Runs on AWS Lambda
  with a Function URL for near-zero cost hosting.

### xtask

Internal tooling invoked by the justfile. Handles build quality, AWS
operations, OpenTofu, deployment, and SQLite backup/restore.

## Design Principles

1. **Zero-cost abstractions**: Generics with monomorphization, not `dyn` dispatch
2. **Trait composition**: Each crate defines traits; implementations are separate crates
3. **thiserror everywhere**: Structured errors with context, no anyhow in libraries
4. **Format agnostic**: Same Rust types work with TOML, YAML, JSON, OpenAPI, GraphQL, gRPC
5. **justfile is the interface**: Developers never need to know about xtask
