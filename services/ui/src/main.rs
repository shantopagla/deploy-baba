use anyhow::Result;
use std::sync::Arc;

mod db;
mod openapi;
mod router;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().without_time().init();

    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "deploy-baba.db".to_string());
    let db = Arc::new(db::Db::open(&db_path)?);
    tracing::info!("→ Database ready at {}", db_path);

    let app = router::build(db);

    if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        tracing::info!("→ Starting as AWS Lambda function");
        lambda_http::run(app)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
    } else {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        tracing::info!("→ http://localhost:3000");
        axum::serve(listener, app).await?;
    }

    Ok(())
}
