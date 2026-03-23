//! Build task implementations
//!
//! Handles compilation, formatting, and code checking tasks

use clap::Subcommand;
use std::process::Command;

#[derive(Subcommand)]
pub enum BuildAction {
    /// Format code with rustfmt
    Format {
        /// Check formatting without making changes (for CI)
        #[arg(long)]
        check: bool,
    },
    /// Lint code with clippy
    Lint {
        /// Auto-fix issues where possible
        #[arg(long)]
        fix: bool,
    },
    /// Compile the project
    Compile {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Features to enable
        #[arg(long)]
        features: Option<String>,
    },
}

pub async fn execute(action: BuildAction) -> anyhow::Result<()> {
    match action {
        BuildAction::Format { check } => format_code(check).await,
        BuildAction::Lint { fix } => lint_code(fix).await,
        BuildAction::Compile { release, features } => compile_code(release, features).await,
    }
}

async fn format_code(check: bool) -> anyhow::Result<()> {
    if check {
        println!("🎨 Checking code formatting...");
        let status = Command::new("cargo")
            .args(["fmt", "--all", "--check"])
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Code formatting check failed"));
        }
        println!("✅ Code formatting is correct");
    } else {
        println!("🎨 Formatting code...");
        let status = Command::new("cargo").args(["fmt", "--all"]).status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Code formatting failed"));
        }
        println!("✅ Code formatted successfully");
    }
    Ok(())
}

async fn lint_code(fix: bool) -> anyhow::Result<()> {
    println!("🔍 Running clippy lints...");
    let mut cmd = Command::new("cargo");
    cmd.args(["clippy", "--workspace", "--all-targets", "--all-features"]);

    if fix {
        cmd.args(["--fix", "--allow-dirty", "--allow-staged"]);
    } else {
        cmd.args(["--", "-D", "warnings"]);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Clippy linting failed"));
    }
    println!("✅ Code linting passed");
    Ok(())
}

async fn compile_code(release: bool, features: Option<String>) -> anyhow::Result<()> {
    println!(
        "🔨 Compiling{}{}...",
        if release { " (release)" } else { "" },
        features
            .as_ref()
            .map(|f| format!(" with features: {}", f))
            .unwrap_or_default()
    );

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--workspace"]);

    if release {
        cmd.arg("--release");
    }

    if let Some(f) = features {
        cmd.args(["--features", &f]);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Compilation failed"));
    }
    println!("✅ Compilation successful");
    Ok(())
}
