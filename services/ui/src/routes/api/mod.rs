pub mod crates;
pub mod demo;
pub mod stack;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/crates", crates::router())
        .nest("/stack", stack::router())
        .nest("/demo", demo::router())
}
