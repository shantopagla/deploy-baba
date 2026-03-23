//! Test execution with comprehensive test categories
//!
//! Implements test runner with categories:
//! - Unit tests (library tests)
//! - Integration tests (test files)

use clap::Subcommand;
use std::process::Command;

#[derive(Subcommand)]
pub enum TestAction {
    /// Run unit tests (library tests only)
    Unit {
        /// Test specific crate
        #[arg(long, short)]
        crate_name: Option<String>,
    },
    /// Run integration tests (test files only)
    Integration {
        /// Test specific crate
        #[arg(long, short)]
        crate_name: Option<String>,
    },
    /// Run all tests
    All {
        /// Test specific crate
        #[arg(long, short)]
        crate_name: Option<String>,
    },
    /// Run a single crate's tests
    Crate {
        /// Crate name to test
        name: String,
    },
}

pub async fn execute(action: TestAction) -> anyhow::Result<()> {
    match action {
        TestAction::Unit { crate_name } => run_unit_tests(crate_name).await,
        TestAction::Integration { crate_name } => run_integration_tests(crate_name).await,
        TestAction::All { crate_name } => run_all_tests(crate_name).await,
        TestAction::Crate { name } => run_crate_tests(&name).await,
    }
}

async fn run_unit_tests(crate_name: Option<String>) -> anyhow::Result<()> {
    println!("🧪 Running unit tests...");

    let mut cmd = Command::new("cargo");
    cmd.args(["test", "--lib"]);

    if let Some(name) = &crate_name {
        println!("   Testing crate: {}", name);
        cmd.args(["--package", name]);
    } else {
        println!("   Testing all crates");
        cmd.arg("--workspace");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Unit tests failed"));
    }
    println!("✅ Unit tests passed");
    Ok(())
}

async fn run_integration_tests(crate_name: Option<String>) -> anyhow::Result<()> {
    println!("🔗 Running integration tests...");

    let mut cmd = Command::new("cargo");
    cmd.args(["test", "--test", "*"]);

    if let Some(name) = &crate_name {
        println!("   Testing crate: {}", name);
        cmd.args(["--package", name]);
    } else {
        println!("   Testing all crates");
        cmd.arg("--workspace");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Integration tests failed"));
    }
    println!("✅ Integration tests passed");
    Ok(())
}

async fn run_all_tests(crate_name: Option<String>) -> anyhow::Result<()> {
    println!("🧪 Running all tests...");

    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(name) = &crate_name {
        println!("   Testing crate: {}", name);
        cmd.args(["--package", name]);
    } else {
        println!("   Testing all crates");
        cmd.arg("--workspace");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Tests failed"));
    }
    println!("✅ All tests passed");
    Ok(())
}

async fn run_crate_tests(crate_name: &str) -> anyhow::Result<()> {
    println!("🧪 Running tests for crate: {}", crate_name);

    let status = Command::new("cargo")
        .args(["test", "--package", crate_name])
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Tests failed for crate: {}", crate_name));
    }
    println!("✅ Tests passed for crate: {}", crate_name);
    Ok(())
}
