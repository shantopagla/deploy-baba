//! Amazon ECS (Elastic Container Service) deployment

use aws_sdk_ecs::Client as EcsClient;

pub async fn deploy(cluster: Option<String>, service: Option<String>) -> anyhow::Result<()> {
    let cluster_name = cluster.unwrap_or_else(|| "deploy-baba-cluster".to_string());
    let service_name = service.unwrap_or_else(|| "deploy-baba-service".to_string());

    println!("🚀 Deploying to Amazon ECS");
    println!("   Cluster: {}", cluster_name);
    println!("   Service: {}", service_name);

    let config = crate::aws::create_aws_config(None).await?;
    let client = EcsClient::new(&config);

    // Update service to force deployment
    println!("   Updating ECS service...");
    client
        .update_service()
        .cluster(&cluster_name)
        .service(&service_name)
        .force_new_deployment(true)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update ECS service: {}", e))?;

    println!("✅ ECS service deployment initiated");
    println!("   Monitor progress in the AWS console");
    Ok(())
}
