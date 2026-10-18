use anyhow::{Context, bail};
use ca_application::{IncidentInput, execute};
use ca_config::Config;
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long, default_value = "target/continuity-state")]
    state: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let a = Args::parse();
    if !ca_cli::safe_state_path(&a.state) {
        bail!("unsafe state path")
    };
    let raw =
        std::fs::read_to_string(&a.input).with_context(|| format!("read {}", a.input.display()))?;
    let input: IncidentInput = serde_json::from_str(&raw)?;
    std::fs::create_dir_all(&a.state)?;
    let out = execute(&input, &a.state, Config::default())?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
