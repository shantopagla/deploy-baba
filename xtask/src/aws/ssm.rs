//! AWS Systems Manager (SSM) parameter operations

use aws_sdk_ssm::Client as SsmClient;

pub async fn get_parameter(name: &str, profile: Option<String>) -> anyhow::Result<String> {
    println!("📖 Getting SSM parameter: {}", name);

    let config = super::create_aws_config(profile).await?;
    let client = SsmClient::new(&config);

    let response = client
        .get_parameter()
        .name(name)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get parameter '{}': {}", name, e))?;

    let value = response
        .parameter
        .and_then(|p| p.value)
        .ok_or_else(|| anyhow::anyhow!("Parameter '{}' has no value", name))?;

    println!("✅ Retrieved parameter: {}", name);
    Ok(value)
}

pub async fn set_parameter(
    name: &str,
    value: &str,
    profile: Option<String>,
) -> anyhow::Result<()> {
    println!("📝 Setting SSM parameter: {}", name);

    let config = super::create_aws_config(profile).await?;
    let client = SsmClient::new(&config);

    client
        .put_parameter()
        .name(name)
        .value(value)
        .overwrite(true)
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to set parameter '{}': {}", name, e))?;

    println!("✅ Parameter set: {}", name);
    Ok(())
}
