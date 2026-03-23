//! Terraform command wrappers
//!
//! All commands default to running in the "infra/" subdirectory via terraform's
//! `-chdir=<dir>` flag. Pass `AWS_PROFILE` via the profile argument so terraform
//! inherits the correct credentials.

use std::process::Command;

fn make_cmd(dir: Option<String>, profile: Option<String>) -> (Command, String) {
    let dir = dir.unwrap_or_else(|| "infra".to_string());
    let mut cmd = Command::new("terraform");
    cmd.arg(format!("-chdir={}", dir));
    if let Some(p) = profile {
        cmd.env("AWS_PROFILE", p);
    }
    (cmd, dir)
}

pub async fn run_terraform_init(
    dir: Option<String>,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("🔧 Initializing Terraform ({})...", dir);
    cmd.arg("init");

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform init failed"));
    }

    println!("✅ Terraform initialized");
    Ok(())
}

pub async fn run_terraform_plan(
    dir: Option<String>,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("📋 Planning Terraform ({})...", dir);
    cmd.arg("plan");

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform plan failed"));
    }

    println!("✅ Terraform plan complete");
    Ok(())
}

pub async fn run_terraform_apply(
    dir: Option<String>,
    auto_approve: bool,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("🚀 Applying Terraform ({})...", dir);
    cmd.arg("apply");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform apply failed"));
    }

    println!("✅ Terraform applied successfully");
    Ok(())
}

pub async fn run_terraform_destroy(
    dir: Option<String>,
    auto_approve: bool,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("💥 Destroying Terraform ({})...", dir);
    cmd.arg("destroy");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform destroy failed"));
    }

    println!("✅ Terraform destroyed");
    Ok(())
}

pub async fn run_terraform_output(
    name: Option<String>,
    dir: Option<String>,
    profile: Option<String>,
) -> anyhow::Result<()> {
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!(
        "📤 Getting Terraform output{} ({})...",
        name.as_ref()
            .map(|n| format!(": {}", n))
            .unwrap_or_default(),
        dir,
    );
    cmd.arg("output");
    cmd.arg("-json");

    if let Some(n) = name {
        cmd.arg(n);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform output failed"));
    }

    println!("✅ Output retrieved");
    Ok(())
}
