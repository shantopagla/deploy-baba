use axum::response::Redirect;

pub async fn handler() -> Redirect {
    Redirect::permanent("/rapidoc")
}
