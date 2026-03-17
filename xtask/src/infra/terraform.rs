//! Terraform command wrappers

use std::process::Command;

pub async fn run_terraform_init(dir: Option<String>) -> anyhow::Result<()> {
    println!("🔧 Initializing Terraform{}...", dir.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default());

    let mut cmd = Command::new("terraform");
    cmd.arg("init");

    if let Some(d) = dir {
        cmd.arg(d);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform init failed"));
    }

    println!("✅ Terraform initialized");
    Ok(())
}

pub async fn run_terraform_plan(dir: Option<String>) -> anyhow::Result<()> {
    println!("📋 Planning Terraform{}...", dir.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default());

    let mut cmd = Command::new("terraform");
    cmd.arg("plan");

    if let Some(d) = dir {
        cmd.arg(d);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform plan failed"));
    }

    println!("✅ Terraform plan complete");
    Ok(())
}

pub async fn run_terraform_apply(dir: Option<String>, auto_approve: bool) -> anyhow::Result<()> {
    println!("🚀 Applying Terraform{}...", dir.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default());

    let mut cmd = Command::new("terraform");
    cmd.arg("apply");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    if let Some(d) = dir {
        cmd.arg(d);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform apply failed"));
    }

    println!("✅ Terraform applied successfully");
    Ok(())
}

pub async fn run_terraform_destroy(dir: Option<String>, auto_approve: bool) -> anyhow::Result<()> {
    println!("💥 Destroying Terraform{}...", dir.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default());

    let mut cmd = Command::new("terraform");
    cmd.arg("destroy");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    if let Some(d) = dir {
        cmd.arg(d);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform destroy failed"));
    }

    println!("✅ Terraform destroyed");
    Ok(())
}

pub async fn run_terraform_output(name: Option<String>, dir: Option<String>) -> anyhow::Result<()> {
    println!("📤 Getting Terraform output{}{}...", 
        name.as_ref().map(|n| format!(": {}", n)).unwrap_or_default(),
        dir.as_ref().map(|d| format!(" ({})", d)).unwrap_or_default()
    );

    let mut cmd = Command::new("terraform");
    cmd.arg("output");

    if let Some(n) = name {
        cmd.arg(n);
    }

    if let Some(d) = dir {
        cmd.arg(d);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Terraform output failed"));
    }

    println!("✅ Output retrieved");
    Ok(())
}
