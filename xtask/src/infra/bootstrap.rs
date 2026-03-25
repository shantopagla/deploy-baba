//! AWS account bootstrap procedures
//!
//! Creates the Terraform remote state backend (S3 bucket + DynamoDB lock table),
//! applies security hardening (encryption, public-access block), writes an SSM
//! sentinel parameter, then runs `terraform init` to wire up the backend.

use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_ssm::Client as SsmClient;

const BUCKET_NAME: &str = "deploy-baba-tfstate";
const LOCK_TABLE: &str = "terraform-lock";
const SENTINEL_PARAM: &str = "/deploy-baba/sentinel";
const SENTINEL_VALUE: &str = "deploy-baba-configured";

pub async fn bootstrap_account(
    profile: Option<String>,
    region: Option<String>,
) -> anyhow::Result<()> {
    println!("🚀 Bootstrapping AWS account for deploy-baba...");

    // Propagate region to subprocesses (terraform init reads AWS_DEFAULT_REGION)
    if let Some(ref r) = region {
        std::env::set_var("AWS_DEFAULT_REGION", r);
    }

    let config = crate::aws::create_aws_config(profile.clone()).await?;

    // S3 state bucket
    create_state_bucket(&config).await?;
    enable_bucket_versioning(&config).await?;
    enable_bucket_encryption(&config).await?;
    block_public_access(&config).await?;

    // DynamoDB lock table
    create_lock_table(&config).await?;

    // SSM sentinel
    create_sentinel_parameter(&config).await?;

    // Wire up the Terraform backend
    println!("   Running terraform init...");
    crate::infra::terraform::run_terraform_init(None, profile).await?;

    println!("✅ AWS account bootstrap complete");
    Ok(())
}

async fn create_state_bucket(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Creating S3 state bucket ({})...", BUCKET_NAME);

    let client = S3Client::new(config);

    if client
        .head_bucket()
        .bucket(BUCKET_NAME)
        .send()
        .await
        .is_ok()
    {
        println!("   ℹ️  Bucket already exists: {}", BUCKET_NAME);
        return Ok(());
    }

    client
        .create_bucket()
        .bucket(BUCKET_NAME)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create S3 bucket: {}", e))?;

    println!("   ✅ Created S3 bucket: {}", BUCKET_NAME);
    Ok(())
}

async fn enable_bucket_versioning(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Enabling S3 versioning...");

    let client = S3Client::new(config);

    let versioning = aws_sdk_s3::types::VersioningConfiguration::builder()
        .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
        .build();

    client
        .put_bucket_versioning()
        .bucket(BUCKET_NAME)
        .versioning_configuration(versioning)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to enable versioning: {}", e))?;

    println!("   ✅ Versioning enabled");
    Ok(())
}

async fn enable_bucket_encryption(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Enabling S3 server-side encryption (AES256)...");

    let client = S3Client::new(config);

    let sse_default = aws_sdk_s3::types::ServerSideEncryptionByDefault::builder()
        .sse_algorithm(aws_sdk_s3::types::ServerSideEncryption::Aes256)
        .build()
        .map_err(|e| anyhow::anyhow!("SSE config error: {}", e))?;

    let rule = aws_sdk_s3::types::ServerSideEncryptionRule::builder()
        .apply_server_side_encryption_by_default(sse_default)
        .build();

    let sse_config = aws_sdk_s3::types::ServerSideEncryptionConfiguration::builder()
        .rules(rule)
        .build()
        .map_err(|e| anyhow::anyhow!("SSE configuration error: {}", e))?;

    client
        .put_bucket_encryption()
        .bucket(BUCKET_NAME)
        .server_side_encryption_configuration(sse_config)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to enable encryption: {}", e))?;

    println!("   ✅ Encryption enabled");
    Ok(())
}

async fn block_public_access(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Blocking public S3 access...");

    let client = S3Client::new(config);

    let block_config = aws_sdk_s3::types::PublicAccessBlockConfiguration::builder()
        .block_public_acls(true)
        .ignore_public_acls(true)
        .block_public_policy(true)
        .restrict_public_buckets(true)
        .build();

    client
        .put_public_access_block()
        .bucket(BUCKET_NAME)
        .public_access_block_configuration(block_config)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to block public access: {}", e))?;

    println!("   ✅ Public access blocked");
    Ok(())
}

async fn create_lock_table(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Creating DynamoDB lock table ({})...", LOCK_TABLE);

    let client = DynamoClient::new(config);

    // Check if table already exists
    if client
        .describe_table()
        .table_name(LOCK_TABLE)
        .send()
        .await
        .is_ok()
    {
        println!("   ℹ️  DynamoDB table already exists: {}", LOCK_TABLE);
        return Ok(());
    }

    let attr_def = aws_sdk_dynamodb::types::AttributeDefinition::builder()
        .attribute_name("LockID")
        .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
        .build()
        .map_err(|e| anyhow::anyhow!("DynamoDB attr definition error: {}", e))?;

    let key_schema = aws_sdk_dynamodb::types::KeySchemaElement::builder()
        .attribute_name("LockID")
        .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
        .build()
        .map_err(|e| anyhow::anyhow!("DynamoDB key schema error: {}", e))?;

    client
        .create_table()
        .table_name(LOCK_TABLE)
        .attribute_definitions(attr_def)
        .key_schema(key_schema)
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create DynamoDB table: {}", e))?;

    println!("   ✅ DynamoDB lock table created: {}", LOCK_TABLE);
    Ok(())
}

async fn create_sentinel_parameter(config: &aws_config::SdkConfig) -> anyhow::Result<()> {
    println!("   Writing SSM sentinel parameter...");

    let client = SsmClient::new(config);

    client
        .put_parameter()
        .name(SENTINEL_PARAM)
        .value(SENTINEL_VALUE)
        .overwrite(true)
        .r#type(aws_sdk_ssm::types::ParameterType::String)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to write sentinel parameter: {}", e))?;

    println!(
        "   ✅ Sentinel parameter written: {} = {}",
        SENTINEL_PARAM, SENTINEL_VALUE
    );
    Ok(())
}
