//! Database backup operations

use aws_sdk_s3::Client as S3Client;
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn backup_database(path: Option<String>, profile: Option<String>) -> anyhow::Result<()> {
    let db_path = path.unwrap_or_else(|| "app.db".to_string());
    println!("💾 Backing up database: {}", db_path);

    if !Path::new(&db_path).exists() {
        return Err(anyhow::anyhow!("Database file not found: {}", db_path));
    }

    // Read database file
    let mut db_file = File::open(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to open database file: {}", e))?;

    let mut db_data = Vec::new();
    db_file
        .read_to_end(&mut db_data)
        .map_err(|e| anyhow::anyhow!("Failed to read database file: {}", e))?;

    // Compress with gzip
    println!("   Compressing...");
    let compressed = compress_data(&db_data)?;

    // Upload to S3
    println!("   Uploading to S3...");
    let config = crate::aws::create_aws_config(profile).await?;
    let client = S3Client::new(&config);

    // Generate timestamp using SystemTime
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow::anyhow!("Failed to get current time: {}", e))?
        .as_secs();
    
    let backup_key = format!("db-backups/app-{}.db.gz", timestamp);

    client
        .put_object()
        .bucket("deploy-baba-backups")
        .key(&backup_key)
        .body(aws_sdk_s3::primitives::ByteStream::from(compressed))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to upload backup to S3: {}", e))?;

    println!("✅ Database backed up: {}", backup_key);
    Ok(())
}

fn compress_data(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|e| anyhow::anyhow!("Compression failed: {}", e))?;

    encoder
        .finish()
        .map_err(|e| anyhow::anyhow!("Compression finish failed: {}", e))
}
