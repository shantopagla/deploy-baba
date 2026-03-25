pub mod competencies;
pub mod crates;
pub mod demo;
pub mod jobs;
pub mod stack;

use axum::Router;
use std::sync::Arc;

use crate::db::Db;

pub fn router() -> Router<Arc<Db>> {
    Router::new()
        .nest("/crates", crates::router())
        .nest("/stack", stack::router())
        .nest("/demo", demo::router())
        .nest("/jobs", jobs::router())
        .nest("/competencies", competencies::router())
}
