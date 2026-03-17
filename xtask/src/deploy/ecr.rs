//! Amazon ECR (Elastic Container Registry) operations

use aws_sdk_ecr::Client as EcrClient;
use std::process::Command;

pub async fn push(image_uri: &str, profile: Option<String>) -> anyhow::Result<()> {
    println!("📤 Pushing image to Amazon ECR...");
    println!("   Image: {}", image_uri);

    let config = crate::aws::create_aws_config(profile).await?;
    let client = EcrClient::new(&config);

    // Get authorization token
    println!("   Getting ECR authorization...");
    let auth_response = client
        .get_authorization_token()
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get ECR auth: {}", e))?;

    let auth_data = auth_response
        .authorization_data
        .and_then(|mut data| data.pop())
        .ok_or_else(|| anyhow::anyhow!("No authorization data received"))?;

    let auth_token = auth_data
        .authorization_token
        .ok_or_else(|| anyhow::anyhow!("No authorization token in response"))?;

    // Decode base64 token to get username:password
    let decoded = base64_decode(&auth_token)?;
    let parts: Vec<&str> = decoded.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid authorization token format"));
    }

    let username = parts[0];
    let password = parts[1];

    // Extract registry URL from image URI
    let registry = image_uri
        .split('/')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid image URI format"))?;

    // Login to ECR via docker login --password-stdin
    println!("   Logging into ECR...");
    let mut child = Command::new("docker")
        .args(["login", "-u", username, "--password-stdin", registry])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn docker login: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        use std::io::Write;
        stdin.write_all(password.as_bytes())?;
    }
    child.stdin.take(); // drop to signal EOF

    let login_output = child.wait_with_output()?;
    if !login_output.status.success() {
        let stderr = String::from_utf8_lossy(&login_output.stderr);
        return Err(anyhow::anyhow!("ECR login failed: {}", stderr));
    }

    // Push image
    println!("   Pushing image...");
    let status = Command::new("docker")
        .args(&["push", image_uri])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Docker push failed"));
    }

    println!("✅ Image pushed to ECR: {}", image_uri);
    Ok(())
}

fn base64_decode(input: &str) -> anyhow::Result<String> {
    // Use the system `base64` command via a single piped invocation
    use std::io::Write;
    let mut child = std::process::Command::new("base64")
        .arg("--decode")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn base64: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(input.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to write to base64 stdin: {}", e))?;
    }
    // Drop stdin to signal EOF
    child.stdin.take();

    let output = child.wait_with_output()
        .map_err(|e| anyhow::anyhow!("Failed to wait for base64: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("base64 decode failed: {}", stderr));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("base64 output is not valid UTF-8: {}", e))
}
