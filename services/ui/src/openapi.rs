use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "deploy-baba Portfolio & API",
        version = "0.1.0",
        description = "Live demos and documentation for the deploy-baba ecosystem"
    ),
    paths(
        crate::routes::health::get_health,
        crate::routes::api::crates::list_crates,
        crate::routes::api::crates::get_crate,
        crate::routes::api::stack::get_stack,
        crate::routes::api::demo::parse_config,
        crate::routes::api::demo::generate_spec,
    ),
    tags(
        (name = "health", description = "Service health checks"),
        (name = "crates", description = "deploy-baba crate information"),
        (name = "stack", description = "Stack configuration examples"),
        (name = "demo", description = "Live API demonstrations"),
    ),
)]
pub struct ApiDoc;
