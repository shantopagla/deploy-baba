//! AWS profile validation
//!
//! Validates AWS credentials and connectivity

use aws_sdk_sts::Client as StsClient;

pub async fn validate_profile(profile: Option<String>) -> anyhow::Result<()> {
    println!(
        "🔐 Validating AWS profile{}",
        profile
            .as_ref()
            .map(|p| format!(": {}", p))
            .unwrap_or_default()
    );

    // Load AWS config
    let config = super::create_aws_config(profile.clone()).await?;

    // Create STS client
    let client = StsClient::new(&config);

    // Call GetCallerIdentity to validate credentials
    let response = client
        .get_caller_identity()
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to validate AWS credentials: {}", e))?;

    println!("✅ AWS credentials valid");
    println!("   Account: {}", response.account.unwrap_or_default());
    println!(
        "   User/Role: {}",
        response.arn.unwrap_or_else(|| "unknown".to_string())
    );

    // Try to read deploy-baba sentinel parameter to verify SSM access
    if let Ok(value) = crate::aws::ssm::get_parameter("/deploy-baba/sentinel", profile).await {
        println!("✅ SSM parameter access confirmed");
        println!("   Sentinel value: {}", value);
    } else {
        println!("⚠️  Could not read /deploy-baba/sentinel parameter (may not exist)");
    }

    println!("✅ AWS profile validation complete");
    Ok(())
}
