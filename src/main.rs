use anyhow::Result;
use clap::Parser;
use guidebook_todo::{run_command, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run_command(cli).await
}
