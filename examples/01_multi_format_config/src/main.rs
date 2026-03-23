//! Example 1: Multi-Format Configuration Parsing
//!
//! This example demonstrates the universal configuration trait interface
//! across TOML, YAML, and JSON formats. It shows how the same AppConfig struct
//! can be parsed from all three formats using zero-cost abstraction through
//! trait-based polymorphism.

use config_core::{ConfigParser, ValidationError};
use config_json::{JsonParser, JsonValidatable};
use config_toml::{TomlParser, TomlValidatable};
use config_yaml::{YamlParser, YamlValidatable};
use serde::{Deserialize, Serialize};

/// Sample application configuration struct
///
/// This struct is parsed from different formats (TOML, YAML, JSON) while
/// maintaining type safety and validation at the trait level.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AppConfig {
    app_name: String,
    version: String,
    port: u16,
    enable_logging: bool,
    max_connections: u32,
}

// Implement validation traits for each format
impl TomlValidatable for AppConfig {
    fn validate_toml(&self) -> Result<(), Vec<ValidationError>> {
        validate_app_config(self)
    }
}

impl YamlValidatable for AppConfig {
    fn validate_yaml(&self) -> Result<(), Vec<ValidationError>> {
        validate_app_config(self)
    }
}

impl JsonValidatable for AppConfig {
    fn validate_json(&self) -> Result<(), Vec<ValidationError>> {
        validate_app_config(self)
    }
}

/// Shared validation logic for AppConfig
fn validate_app_config(config: &AppConfig) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if config.app_name.is_empty() {
        errors.push(ValidationError::new(
            "app_name",
            "Application name cannot be empty",
        ));
    }

    if config.port == 0 {
        errors.push(ValidationError::new("port", "Port must be non-zero"));
    }

    if config.max_connections == 0 {
        errors.push(ValidationError::new(
            "max_connections",
            "Max connections must be at least 1",
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multi-Format Configuration Example ===\n");

    // TOML Configuration
    println!("1. Parsing TOML Configuration");
    println!("---------------------------------");
    let toml_content = r#"
app_name = "MyApplication"
version = "1.0.0"
port = 8080
enable_logging = true
max_connections = 100
"#;
    println!("TOML Content:\n{}\n", toml_content);

    let toml_config: AppConfig = TomlParser::parse_and_validate(toml_content)?;
    println!("Parsed TOML Config: {:#?}\n", toml_config);

    // YAML Configuration
    println!("2. Parsing YAML Configuration");
    println!("---------------------------------");
    let yaml_content = r#"
app_name: MyApplication
version: 1.0.0
port: 8080
enable_logging: true
max_connections: 100
"#;
    println!("YAML Content:\n{}\n", yaml_content);

    let yaml_config: AppConfig = YamlParser::parse_and_validate(yaml_content)?;
    println!("Parsed YAML Config: {:#?}\n", yaml_config);

    // JSON Configuration
    println!("3. Parsing JSON Configuration");
    println!("---------------------------------");
    let json_content = r#"{
  "app_name": "MyApplication",
  "version": "1.0.0",
  "port": 8080,
  "enable_logging": true,
  "max_connections": 100
}"#;
    println!("JSON Content:\n{}\n", json_content);

    let json_config: AppConfig = JsonParser::parse_and_validate(json_content)?;
    println!("Parsed JSON Config: {:#?}\n", json_config);

    // Verify all three formats produce the same result
    println!("4. Verifying Format Consistency");
    println!("---------------------------------");
    assert_eq!(toml_config, yaml_config, "TOML and YAML configs differ");
    assert_eq!(yaml_config, json_config, "YAML and JSON configs differ");
    println!("✓ All three formats produce identical configurations\n");

    // Demonstrate validation errors
    println!("5. Demonstrating Validation Errors");
    println!("----------------------------------");
    let invalid_toml = r#"
app_name = ""
version = "1.0.0"
port = 0
enable_logging = true
max_connections = 0
"#;
    println!(
        "Invalid TOML (empty app_name, port=0, max_connections=0):\n{}",
        invalid_toml
    );

    let toml_result: Result<AppConfig, _> = TomlParser::parse_and_validate(invalid_toml);
    match toml_result {
        Ok(_) => println!("ERROR: Validation should have failed!"),
        Err(e) => println!("✓ Validation correctly failed: {}\n", e),
    }

    // Show that the same validation happens with different formats
    let invalid_yaml = r#"
app_name: ""
version: 1.0.0
port: 0
enable_logging: true
max_connections: 0
"#;

    let yaml_result: Result<AppConfig, _> = YamlParser::parse_and_validate(invalid_yaml);
    match yaml_result {
        Ok(_) => println!("ERROR: Validation should have failed!"),
        Err(e) => println!("✓ YAML validation correctly failed: {}\n", e),
    }

    // Summary
    println!("6. Summary");
    println!("----------");
    println!("This example demonstrated:");
    println!("• Universal trait interface across three formats (TOML, YAML, JSON)");
    println!("• Zero-cost abstraction through monomorphization");
    println!("• Consistent validation across all formats");
    println!("• Same struct type parsed from different serialization formats");

    Ok(())
}
