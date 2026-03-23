//! Quality gate enforcement
//!
//! Orchestrates: fmt-check -> lint -> test -> coverage -> audit

use clap::Subcommand;

#[derive(Subcommand)]
pub enum QualityAction {
    /// Run all quality gates in sequence
    All,
    /// Run format checks
    Format,
    /// Run linting
    Lint,
    /// Run tests
    Test,
    /// Run coverage checks
    Coverage,
}

pub async fn execute(action: QualityAction) -> anyhow::Result<()> {
    match action {
        QualityAction::All => run_all_gates().await,
        QualityAction::Format => crate::build::execute(crate::build::BuildAction::Format { check: true }).await,
        QualityAction::Lint => crate::build::execute(crate::build::BuildAction::Lint { fix: false }).await,
        QualityAction::Test => crate::test::execute(crate::test::TestAction::All { crate_name: None }).await,
        QualityAction::Coverage => crate::coverage::execute(crate::coverage::CoverageAction::Floors).await,
    }
}

async fn run_all_gates() -> anyhow::Result<()> {
    println!("🔍 Running all quality gates...\n");

    // 1. Format check
    println!("📐 Step 1: Code formatting");
    crate::build::execute(crate::build::BuildAction::Format { check: true }).await?;
    println!();

    // 2. Lint
    println!("🔍 Step 2: Linting");
    crate::build::execute(crate::build::BuildAction::Lint { fix: false }).await?;
    println!();

    // 3. Tests
    println!("🧪 Step 3: Running tests");
    crate::test::execute(crate::test::TestAction::All { crate_name: None }).await?;
    println!();

    // 4. Coverage
    println!("📊 Step 4: Coverage check");
    crate::coverage::execute(crate::coverage::CoverageAction::Floors).await?;
    println!();

    // 5. Security audit
    println!("🔒 Step 5: Security audit");
    let audit_status = std::process::Command::new("cargo")
        .args(["audit"])
        .status()?;
    if !audit_status.success() {
        return Err(anyhow::anyhow!("Security audit failed"));
    }
    println!();

    println!("✅ All quality gates passed!");
    Ok(())
}
