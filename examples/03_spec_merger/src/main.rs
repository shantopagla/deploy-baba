//! Example 3: API Specification Merger
//!
//! This example demonstrates merging multiple OpenAPI specifications with different
//! conflict resolution strategies (KeepFirst, KeepLast, Error). It shows how the
//! SpecificationMerger handles overlapping paths and schemas across multiple services.

use api_core::SpecFormat;
use api_merger::{ConflictResolutionStrategy, SpecificationMerger, UnifiedApiSpec};
use api_openapi::OpenApiSpec;
use serde::Deserialize;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

// ============ User Service Specification ============

#[derive(Serialize, Deserialize, ToSchema)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]
async fn get_users() -> Vec<User> {
    vec![]
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
async fn create_user(_user: User) -> User {
    User {
        id: 1,
        name: "Test".to_string(),
        email: "test@example.com".to_string(),
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user),
    components(schemas(User))
)]
struct UserServiceSpec;

impl api_openapi::OpenApiSchema for UserServiceSpec {
    fn api_schema() -> utoipa::openapi::OpenApi {
        UserServiceSpec::openapi()
    }
}

// ============ Product Service Specification ============

#[derive(Serialize, Deserialize, ToSchema)]
struct Product {
    id: u32,
    name: String,
    #[schema(example = 29.99)]
    price: f64,
}

#[utoipa::path(
    get,
    path = "/products",
    responses(
        (status = 200, description = "List products", body = [Product])
    )
)]
async fn get_products() -> Vec<Product> {
    vec![]
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = Product,
    responses(
        (status = 201, description = "Product created", body = Product)
    )
)]
async fn create_product(_product: Product) -> Product {
    Product {
        id: 1,
        name: "Test Product".to_string(),
        price: 29.99,
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_products, create_product),
    components(schemas(Product))
)]
struct ProductServiceSpec;

impl api_openapi::OpenApiSchema for ProductServiceSpec {
    fn api_schema() -> utoipa::openapi::OpenApi {
        ProductServiceSpec::openapi()
    }
}

// ============ Overlapping Service (Conflicting Paths) ============

#[derive(Serialize, Deserialize, ToSchema)]
struct Order {
    id: u32,
    user_id: u32,
    product_id: u32,
}

#[utoipa::path(
    get,
    path = "/orders",
    responses(
        (status = 200, description = "List orders", body = [Order])
    )
)]
async fn get_orders() -> Vec<Order> {
    vec![]
}

#[derive(OpenApi)]
#[openapi(
    paths(get_orders),
    components(schemas(Order))
)]
struct OrderServiceSpec;

impl api_openapi::OpenApiSchema for OrderServiceSpec {
    fn api_schema() -> utoipa::openapi::OpenApi {
        OrderServiceSpec::openapi()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== API Specification Merger Example ===\n");

    // Create specifications for each service
    println!("1. Creating Service Specifications");
    println!("----------------------------------");

    let user_spec = UserServiceSpec::openapi();
    let product_spec = ProductServiceSpec::openapi();
    let order_spec = OrderServiceSpec::openapi();

    println!("✓ User Service: {} paths", user_spec.paths.paths.len());
    println!("✓ Product Service: {} paths", product_spec.paths.paths.len());
    println!("✓ Order Service: {} paths\n", order_spec.paths.paths.len());

    // Wrap in unified API spec
    let user_spec_unified = UnifiedApiSpec::OpenApi(Box::new(OpenApiSpec {
        openapi: user_spec,
        metadata: api_openapi::OpenApiMetadata {
            generator: "example".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            validated: true,
            path_count: 2,
            schema_count: 1,
        },
    }));

    let product_spec_unified = UnifiedApiSpec::OpenApi(Box::new(OpenApiSpec {
        openapi: product_spec,
        metadata: api_openapi::OpenApiMetadata {
            generator: "example".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            validated: true,
            path_count: 2,
            schema_count: 1,
        },
    }));

    let order_spec_unified = UnifiedApiSpec::OpenApi(Box::new(OpenApiSpec {
        openapi: order_spec,
        metadata: api_openapi::OpenApiMetadata {
            generator: "example".to_string(),
            generated_at: "2025-01-01T00:00:00Z".to_string(),
            validated: true,
            path_count: 1,
            schema_count: 1,
        },
    }));

    // Test 1: Merge without conflicts (User + Product + Order)
    println!("2. Merge Without Conflicts");
    println!("---------------------------");

    let specs_no_conflict = vec![
        user_spec_unified.clone(),
        product_spec_unified.clone(),
        order_spec_unified.clone(),
    ];

    let merger = SpecificationMerger::new(SpecFormat::OpenApi)
        .with_conflict_resolution(ConflictResolutionStrategy::FailOnConflict);

    match merger.merge_specifications(specs_no_conflict) {
        Ok(merged) => {
            println!("✓ Merge successful!");
            println!("  Format: {:?}", merged.metadata.format);
            println!("  Source specs: {}", merged.metadata.source_count);
            println!("  Conflicts: {}\n", merged.metadata.conflicts.len());
        }
        Err(e) => println!("✗ Merge failed: {}\n", e),
    }

    // Test 2: Merge with FirstWins strategy
    println!("3. Merge with FirstWins Strategy");
    println!("--------------------------------");

    let specs_for_first_wins = vec![
        user_spec_unified.clone(),
        product_spec_unified.clone(),
    ];

    let merger_first_wins = SpecificationMerger::new(SpecFormat::OpenApi)
        .with_conflict_resolution(ConflictResolutionStrategy::FirstWins)
        .with_validation(true);

    match merger_first_wins.merge_specifications(specs_for_first_wins) {
        Ok(merged) => {
            println!("✓ Merge with FirstWins strategy successful!");
            println!("  Resolution strategy: {}", merged.metadata.resolution_strategy);
            println!("  Validated: {}\n", merged.metadata.validated);
        }
        Err(e) => println!("✗ Merge failed: {}\n", e),
    }

    // Test 3: Merge with LastWins strategy
    println!("4. Merge with LastWins Strategy");
    println!("-------------------------------");

    let specs_for_last_wins = vec![
        product_spec_unified.clone(),
        order_spec_unified.clone(),
    ];

    let merger_last_wins = SpecificationMerger::new(SpecFormat::OpenApi)
        .with_conflict_resolution(ConflictResolutionStrategy::LastWins);

    match merger_last_wins.merge_specifications(specs_for_last_wins) {
        Ok(merged) => {
            println!("✓ Merge with LastWins strategy successful!");
            println!("  Resolution strategy: {}", merged.metadata.resolution_strategy);
            println!("  Conflicts encountered: {}\n", merged.metadata.conflicts.len());
        }
        Err(e) => println!("✗ Merge failed: {}\n", e),
    }

    // Test 4: Show merged specification
    println!("5. Examining Merged Specification");
    println!("--------------------------------");

    let final_merge = vec![
        user_spec_unified,
        product_spec_unified,
        order_spec_unified,
    ];

    let merger_final = SpecificationMerger::new(SpecFormat::OpenApi)
        .with_conflict_resolution(ConflictResolutionStrategy::FailOnConflict);

    if let Ok(merged) = merger_final.merge_specifications(final_merge) {
        let spec_json = merged.spec.content();
        let truncated = if spec_json.len() > 300 {
            format!("{}...", &spec_json[..300])
        } else {
            spec_json
        };
        println!("Merged specification content (truncated):\n{}\n", truncated);
    }

    // Summary
    println!("6. Summary");
    println!("----------");
    println!("This example demonstrated:");
    println!("• Creating independent OpenAPI specifications for multiple services");
    println!("• Merging specifications with different conflict resolution strategies");
    println!("• FailOnConflict: Strict mode, fails on any duplicate");
    println!("• FirstWins: Uses first encountered definition");
    println!("• LastWins: Uses last encountered definition");
    println!("• Tracking merge metadata including conflicts and resolution strategy");
    println!("• Validation before and after merging");

    Ok(())
}
