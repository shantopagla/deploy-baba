//! Docker image building

use std::process::Command;

pub async fn build(platform: &str, tag: Option<String>) -> anyhow::Result<()> {
    println!("🐳 Building Docker image...");
    println!("   Platform: {}", platform);

    let image_tag = tag.unwrap_or_else(|| "deploy-baba-ui:latest".to_string());
    println!("   Tag: {}", image_tag);

    let status = Command::new("docker")
        .args(&["build", "--platform", platform, "-t", &image_tag, "."])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Docker build failed"));
    }

    println!("✅ Docker image built successfully: {}", image_tag);
    Ok(())
}
