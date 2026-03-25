use axum::{response::Html, routing::get, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;

use crate::db::Db;
use crate::openapi::ApiDoc;
use crate::routes;

pub fn build(db: Arc<Db>) -> Router {
    let api_routes = routes::api::router();

    let openapi = ApiDoc::openapi();

    Router::new()
        .route("/", get(routes::landing::handler))
        .route("/health", get(routes::health::get_health))
        .route("/resume", get(routes::resume::handler))
        .nest("/api", api_routes)
        .route("/docs", get(docs_handler))
        .route(
            "/api/openapi.json",
            get(move || async move { axum::Json(openapi) }),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(db)
}

async fn docs_handler() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html><head><meta charset="utf-8"><title>deploy-baba API Docs</title>
<script type="module" src="https://unpkg.com/rapidoc/dist/rapidoc-min.js"></script>
</head><body>
<rapi-doc spec-url="/api/openapi.json" theme="dark" render-style="read"
  show-header="false" allow-try="true"></rapi-doc>
</body></html>"#,
    )
}
