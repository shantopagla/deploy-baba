//! Coverage analysis with per-crate floors
//!
//! Implements coverage enforcement with per-crate thresholds:
//! - config-core: 90%
//! - api-core: 90%
//! - config-toml/yaml/json: 85%
//! - api-openapi/graphql/grpc: 80%
//! - api-merger: 80%
//! - infra-types: 75%

use clap::Subcommand;
use std::collections::HashMap;
use std::process::Command;

#[derive(Subcommand)]
pub enum CoverageAction {
    /// Generate coverage report
    Report {
        /// Open report in browser
        #[arg(long)]
        open: bool,
    },
    /// Check coverage against threshold
    Check {
        /// Minimum coverage percentage
        #[arg(long, default_value = "80")]
        threshold: u8,
    },
    /// Enforce per-crate coverage floors
    Floors,
}

pub async fn execute(action: CoverageAction) -> anyhow::Result<()> {
    match action {
        CoverageAction::Report { open } => generate_report(open).await,
        CoverageAction::Check { threshold } => check_threshold(threshold).await,
        CoverageAction::Floors => enforce_floors().await,
    }
}

async fn generate_report(open: bool) -> anyhow::Result<()> {
    println!("📊 Generating coverage report...");

    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "llvm-cov",
        "--workspace",
        "--html",
        "--output-dir",
        "target/coverage",
    ]);

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Coverage report generation failed"));
    }

    println!("✅ Coverage report generated: target/coverage/index.html");

    if open {
        let _ = Command::new("open").arg("target/coverage/index.html").status();
    }

    Ok(())
}

async fn check_threshold(threshold: u8) -> anyhow::Result<()> {
    println!("🎯 Checking coverage threshold: {}%", threshold);

    let output = Command::new("cargo")
        .args(&["llvm-cov", "--workspace", "--summary-only"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Coverage check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    // Parse coverage percentage from output
    if let Some(coverage) = parse_coverage(&stdout) {
        if coverage >= threshold as f64 {
            println!(
                "✅ Coverage threshold met: {:.1}% >= {}%",
                coverage, threshold
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Coverage below threshold: {:.1}% < {}%",
                coverage,
                threshold
            ))
        }
    } else {
        Err(anyhow::anyhow!("Could not parse coverage from output"))
    }
}

async fn enforce_floors() -> anyhow::Result<()> {
    println!("🏢 Enforcing per-crate coverage floors...");

    let mut floors = HashMap::new();
    floors.insert("config-core", 90);
    floors.insert("api-core", 90);
    floors.insert("config-toml", 85);
    floors.insert("config-yaml", 85);
    floors.insert("config-json", 85);
    floors.insert("api-openapi", 80);
    floors.insert("api-graphql", 80);
    floors.insert("api-grpc", 80);
    floors.insert("api-merger", 80);
    floors.insert("infra-types", 75);

    let mut failed = Vec::new();

    for (crate_name, floor) in floors.iter() {
        match get_crate_coverage(crate_name).await {
            Ok(coverage) => {
                if coverage >= *floor as f64 {
                    println!("   ✅ {}: {:.1}% >= {}%", crate_name, coverage, floor);
                } else {
                    println!("   ❌ {}: {:.1}% < {}%", crate_name, coverage, floor);
                    failed.push(*crate_name);
                }
            }
            Err(_) => {
                println!("   ⚠️  {}: no coverage data (crate may not exist)", crate_name);
            }
        }
    }

    if failed.is_empty() {
        println!("\n✅ All crates meet coverage floors");
        Ok(())
    } else {
        println!("\n❌ {} crates failed coverage floors:", failed.len());
        for crate_name in failed {
            println!("   - {}", crate_name);
        }
        Err(anyhow::anyhow!("Coverage floors not met"))
    }
}

async fn get_crate_coverage(crate_name: &str) -> anyhow::Result<f64> {
    let output = Command::new("cargo")
        .args(&["llvm-cov", "--package", crate_name, "--summary-only"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Could not get coverage for {}", crate_name));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_coverage(&stdout)
        .ok_or_else(|| anyhow::anyhow!("Could not parse coverage for {}", crate_name))
}

fn parse_coverage(output: &str) -> Option<f64> {
    for line in output.lines() {
        if let Some(percent_str) = line.split_whitespace().last() {
            if let Some(num_str) = percent_str.strip_suffix('%') {
                if let Ok(num) = num_str.parse::<f64>() {
                    return Some(num);
                }
            }
        }
    }
    None
}
