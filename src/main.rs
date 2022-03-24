use anyhow::Result;
use clap::Parser;
use nxctl::{do_action, Args};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    do_action(args).await?;
    Ok(())
}
