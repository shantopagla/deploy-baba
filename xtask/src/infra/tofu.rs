//! OpenTofu command wrappers
//!
//! All commands default to running in the "infra/" subdirectory via tofu's
//! `-chdir=<dir>` flag. Pass `AWS_PROFILE` via the profile argument so tofu
//! inherits the correct credentials.

use std::process::Command;

/// Verify the `tofu` binary is available before running any infrastructure command.
pub fn check_tofu_binary() -> anyhow::Result<()> {
    let output = Command::new("tofu").arg("version").output();
    match output {
        Ok(o) if o.status.success() => Ok(()),
        _ => Err(anyhow::anyhow!(
            "tofu binary not found. Install with: brew install opentofu"
        )),
    }
}

fn make_cmd(dir: Option<String>, profile: Option<String>) -> (Command, String) {
    let dir = dir.unwrap_or_else(|| "infra".to_string());
    let mut cmd = Command::new("tofu");
    cmd.arg(format!("-chdir={}", dir));
    if let Some(p) = profile {
        cmd.env("AWS_PROFILE", p);
    }
    (cmd, dir)
}

pub async fn run_tofu_init(dir: Option<String>, profile: Option<String>) -> anyhow::Result<()> {
    check_tofu_binary()?;
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("Initializing OpenTofu ({})...", dir);
    cmd.arg("init");

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("tofu init failed"));
    }

    println!("OpenTofu initialized");
    Ok(())
}

pub async fn run_tofu_plan(dir: Option<String>, profile: Option<String>) -> anyhow::Result<()> {
    check_tofu_binary()?;
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("Planning OpenTofu ({})...", dir);
    cmd.arg("plan");

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("tofu plan failed"));
    }

    println!("OpenTofu plan complete");
    Ok(())
}

pub async fn run_tofu_apply(
    dir: Option<String>,
    auto_approve: bool,
    profile: Option<String>,
) -> anyhow::Result<()> {
    check_tofu_binary()?;
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("Applying OpenTofu ({})...", dir);
    cmd.arg("apply");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("tofu apply failed"));
    }

    println!("OpenTofu applied successfully");
    Ok(())
}

pub async fn run_tofu_destroy(
    dir: Option<String>,
    auto_approve: bool,
    profile: Option<String>,
) -> anyhow::Result<()> {
    check_tofu_binary()?;
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!("Destroying OpenTofu ({})...", dir);
    cmd.arg("destroy");

    if auto_approve {
        cmd.arg("-auto-approve");
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("tofu destroy failed"));
    }

    println!("OpenTofu destroyed");
    Ok(())
}

pub async fn run_tofu_output(
    name: Option<String>,
    dir: Option<String>,
    profile: Option<String>,
) -> anyhow::Result<()> {
    check_tofu_binary()?;
    let (mut cmd, dir) = make_cmd(dir, profile);
    println!(
        "Getting OpenTofu output{} ({})...",
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
        return Err(anyhow::anyhow!("tofu output failed"));
    }

    println!("Output retrieved");
    Ok(())
}
