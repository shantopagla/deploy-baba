use axum::Json;
use serde::Serialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: VERSION.into(),
    })
}
