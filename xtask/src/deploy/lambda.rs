//! AWS Lambda deployment
//!
//! Builds the deploy-baba-ui binary for aarch64 (ARM64) using cargo-lambda,
//! packages it as lambda.zip, and uploads it to the Lambda function.

use aws_sdk_lambda::Client as LambdaClient;
use std::process::Command;

const PACKAGE: &str = "deploy-baba-ui";
const TARGET: &str = "aarch64-unknown-linux-gnu";
const ZIP_PATH: &str = "infra/build/lambda.zip";
const DEFAULT_FUNCTION: &str = "deploy-baba-prod";

pub async fn deploy(function: Option<String>, profile: Option<String>) -> anyhow::Result<()> {
    let func_name = function.unwrap_or_else(|| DEFAULT_FUNCTION.to_string());
    println!("🚀 Deploying to AWS Lambda: {}", func_name);

    // Build release binary for aarch64 using cargo-lambda
    println!("   Building {} ({})...", PACKAGE, TARGET);
    let status = Command::new("cargo")
        .args([
            "lambda",
            "build",
            "--release",
            "--package",
            PACKAGE,
            "--target",
            TARGET,
        ])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo lambda: {} (is cargo-lambda installed?)", e))?;

    if !status.success() {
        return Err(anyhow::anyhow!("cargo lambda build failed"));
    }

    // Package bootstrap binary into lambda.zip
    println!("   Packaging {}...", ZIP_PATH);
    std::fs::create_dir_all("infra/build")
        .map_err(|e| anyhow::anyhow!("Failed to create infra/build: {}", e))?;

    // Remove stale zip to avoid zip appending to old archive
    let _ = std::fs::remove_file(ZIP_PATH);

    let bootstrap_path = format!("target/lambda/{}/bootstrap", PACKAGE);
    let status = Command::new("zip")
        .args(["-j", ZIP_PATH, &bootstrap_path])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run zip: {}", e))?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to create deployment package"));
    }

    // Upload zip to Lambda
    println!("   Uploading to Lambda function: {}...", func_name);
    let config = crate::aws::create_aws_config(profile).await?;
    let client = LambdaClient::new(&config);

    let zip_data = std::fs::read(ZIP_PATH)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", ZIP_PATH, e))?;

    client
        .update_function_code()
        .function_name(&func_name)
        .zip_file(aws_sdk_lambda::primitives::Blob::new(zip_data))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update Lambda function: {}", e))?;

    println!("✅ Lambda function deployed: {}", func_name);
    Ok(())
}
