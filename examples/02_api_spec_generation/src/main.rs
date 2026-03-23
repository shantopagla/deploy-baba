//! Example 2: API Specification Generation
//!
//! This example demonstrates generating API specifications in three different formats
//! (OpenAPI, GraphQL, gRPC) from a single API schema definition. It shows how the
//! universal ApiSpecGenerator trait enables format-agnostic specification generation.

use api_core::ApiSpecGenerator;
use api_graphql::{GraphQLGenerator, GraphQLSchema, GraphQLSchemaDefinition};
use api_grpc::{GrpcGenerator, GrpcSchema, MethodStreaming, ProtoDefinition, ProtoField, ProtoMessage, ProtoMethod, ProtoService};
use api_openapi::{OpenApiGenerator, OpenApiSchema};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

// ============ API Models ============

/// Pet model for the PetStore API
#[derive(Serialize, Deserialize, ToSchema)]
struct Pet {
    id: u32,
    name: String,
    #[schema(example = "dog")]
    species: String,
    #[schema(example = 5)]
    age: u32,
}

/// Request to create a new pet
#[derive(Serialize, Deserialize, ToSchema)]
struct CreatePetRequest {
    name: String,
    species: String,
    age: u32,
}

// ============ OpenAPI Definition ============

/// Mock OpenAPI handler functions (normally would be actual Axum handlers)
#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/pets",
    responses(
        (status = 200, description = "List all pets", body = [Pet])
    )
)]
async fn list_pets() -> Vec<Pet> {
    vec![]
}

#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/pets",
    request_body = CreatePetRequest,
    responses(
        (status = 201, description = "Pet created successfully", body = Pet),
        (status = 400, description = "Invalid request")
    )
)]
async fn create_pet(_request: CreatePetRequest) -> Pet {
    Pet {
        id: 1,
        name: "Fluffy".to_string(),
        species: "cat".to_string(),
        age: 3,
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(list_pets, create_pet),
    components(schemas(Pet, CreatePetRequest))
)]
struct PetStoreOpenApi;

impl api_openapi::OpenApiSchema for PetStoreOpenApi {
    fn api_schema() -> utoipa::openapi::OpenApi {
        PetStoreOpenApi::openapi()
    }
}

// ============ GraphQL Definition ============

struct PetStoreGraphQL;

impl GraphQLSchema for PetStoreGraphQL {
    fn schema_definition() -> GraphQLSchemaDefinition {
        GraphQLSchemaDefinition {
            sdl: r#"
type Query {
    pets: [Pet!]!
    pet(id: ID!): Pet
}

type Pet {
    id: ID!
    name: String!
    species: String!
    age: Int!
}

type Mutation {
    createPet(input: CreatePetInput!): Pet!
}

input CreatePetInput {
    name: String!
    species: String!
    age: Int!
}
"#
            .to_string(),
        }
    }
}

// ============ gRPC Definition ============

struct PetStoreGrpc;

impl GrpcSchema for PetStoreGrpc {
    fn proto_definition() -> ProtoDefinition {
        ProtoDefinition {
            package: "petstore".to_string(),
            messages: vec![
                ProtoMessage::with_fields(
                    "Pet".to_string(),
                    vec![
                        ProtoField {
                            name: "id".to_string(),
                            field_type: "uint32".to_string(),
                            number: 1,
                            optional: false,
                            repeated: false,
                        },
                        ProtoField {
                            name: "name".to_string(),
                            field_type: "string".to_string(),
                            number: 2,
                            optional: false,
                            repeated: false,
                        },
                        ProtoField {
                            name: "species".to_string(),
                            field_type: "string".to_string(),
                            number: 3,
                            optional: false,
                            repeated: false,
                        },
                        ProtoField {
                            name: "age".to_string(),
                            field_type: "uint32".to_string(),
                            number: 4,
                            optional: false,
                            repeated: false,
                        },
                    ],
                ),
                ProtoMessage::with_fields(
                    "CreatePetRequest".to_string(),
                    vec![
                        ProtoField {
                            name: "name".to_string(),
                            field_type: "string".to_string(),
                            number: 1,
                            optional: false,
                            repeated: false,
                        },
                        ProtoField {
                            name: "species".to_string(),
                            field_type: "string".to_string(),
                            number: 2,
                            optional: false,
                            repeated: false,
                        },
                        ProtoField {
                            name: "age".to_string(),
                            field_type: "uint32".to_string(),
                            number: 3,
                            optional: false,
                            repeated: false,
                        },
                    ],
                ),
                ProtoMessage::with_fields("Empty".to_string(), vec![]),
            ],
            services: vec![ProtoService {
                name: "PetStore".to_string(),
                methods: vec![
                    ProtoMethod {
                        name: "ListPets".to_string(),
                        input_type: "Empty".to_string(),
                        output_type: "Pet".to_string(),
                        streaming: MethodStreaming::ServerStreaming,
                    },
                    ProtoMethod {
                        name: "CreatePet".to_string(),
                        input_type: "CreatePetRequest".to_string(),
                        output_type: "Pet".to_string(),
                        streaming: MethodStreaming::Unary,
                    },
                ],
            }],
            imports: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== API Specification Generation Example ===\n");

    // Generate OpenAPI specification
    println!("1. OpenAPI 3.0 Specification");
    println!("----------------------------");
    let openapi_schema = PetStoreOpenApi::api_schema();
    let openapi_spec = OpenApiGenerator::<PetStoreOpenApi>::generate_spec(openapi_schema)?;

    println!("OpenAPI Metadata:");
    println!("  Generator: {}", openapi_spec.metadata.generator);
    println!("  Paths: {}", openapi_spec.metadata.path_count);
    println!("  Schemas: {}\n", openapi_spec.metadata.schema_count);

    let openapi_json = serde_json::to_string_pretty(&openapi_spec.openapi)?;
    println!("OpenAPI JSON (truncated):\n{}\n",
        if openapi_json.len() > 400 {
            format!("{}...", &openapi_json[..400])
        } else {
            openapi_json
        }
    );

    // Generate GraphQL specification
    println!("2. GraphQL Schema Definition Language");
    println!("-------------------------------------");
    let graphql_schema = PetStoreGraphQL::schema_definition();
    let graphql_spec = GraphQLGenerator::<PetStoreGraphQL>::generate_spec(graphql_schema)?;

    println!("GraphQL Metadata:");
    println!("  Generator: {}", graphql_spec.metadata.generator);
    println!("  Types: {}", graphql_spec.metadata.type_count);
    println!("  Queries: {}", graphql_spec.metadata.query_count);
    println!("  Mutations: {}\n", graphql_spec.metadata.mutation_count);

    println!("GraphQL SDL:\n{}\n", graphql_spec.sdl);

    // Generate gRPC specification
    println!("3. Protocol Buffers (gRPC) Specification");
    println!("---------------------------------------");
    let grpc_schema = PetStoreGrpc::proto_definition();
    let grpc_spec = GrpcGenerator::<PetStoreGrpc>::generate_spec(grpc_schema)?;

    println!("gRPC Metadata:");
    println!("  Generator: {}", grpc_spec.metadata.generator);
    println!("  Package: {}", grpc_spec.metadata.package);
    println!("  Messages: {}", grpc_spec.metadata.message_count);
    println!("  Services: {}", grpc_spec.metadata.service_count);
    println!("  Methods: {}\n", grpc_spec.metadata.method_count);

    println!("gRPC Proto File:\n{}\n", grpc_spec.proto_content);

    // Validate each specification
    println!("4. Specification Validation");
    println!("--------------------------");

    match OpenApiGenerator::<PetStoreOpenApi>::validate_spec(&openapi_spec) {
        Ok(_) => println!("✓ OpenAPI specification is valid"),
        Err(errors) => println!("✗ OpenAPI validation errors: {:?}", errors),
    }

    match GraphQLGenerator::<PetStoreGraphQL>::validate_spec(&graphql_spec) {
        Ok(_) => println!("✓ GraphQL specification is valid"),
        Err(errors) => println!("✗ GraphQL validation errors: {:?}", errors),
    }

    match GrpcGenerator::<PetStoreGrpc>::validate_spec(&grpc_spec) {
        Ok(_) => println!("✓ gRPC specification is valid"),
        Err(errors) => println!("✗ gRPC validation errors: {:?}", errors),
    }

    println!("\n5. Summary");
    println!("----------");
    println!("This example demonstrated:");
    println!("• Unified ApiSpecGenerator trait across three formats");
    println!("• OpenAPI (REST) specification generation and validation");
    println!("• GraphQL schema generation with types and operations");
    println!("• Protocol Buffers specification generation for gRPC");
    println!("• Consistent interface for generating different spec formats");

    Ok(())
}
