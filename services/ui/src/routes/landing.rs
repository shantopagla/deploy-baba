use askama::Template;
use askama_axum::IntoResponse;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {
    version: &'static str,
    crate_count: usize,
}

pub async fn handler() -> impl IntoResponse {
    LandingTemplate {
        version: VERSION,
        crate_count: 10,
    }
}
