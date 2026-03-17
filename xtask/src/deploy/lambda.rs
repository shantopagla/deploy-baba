//! AWS Lambda deployment

use aws_sdk_lambda::Client as LambdaClient;
use std::process::Command;

pub async fn deploy(function_name: Option<String>) -> anyhow::Result<()> {
    let func_name = function_name.unwrap_or_else(|| "deploy-baba-api".to_string());
    println!("🚀 Deploying to AWS Lambda: {}", func_name);

    // Build release binary
    println!("   Building release binary...");
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Build failed"));
    }

    // Create deployment package (zip)
    println!("   Creating deployment package...");
    let status = Command::new("zip")
        .args(&["-r", "lambda-deployment.zip", "target/release/"])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to create deployment package"));
    }

    // Upload to Lambda
    println!("   Uploading to Lambda...");
    let config = crate::aws::create_aws_config(None).await?;
    let client = LambdaClient::new(&config);

    // Read zip file
    let zip_data = std::fs::read("lambda-deployment.zip")
        .map_err(|e| anyhow::anyhow!("Failed to read deployment package: {}", e))?;

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
