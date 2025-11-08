use clap::Parser;
use devspin_cli::cli::Cli;
use devspin_cli::error::ToolError;

#[tokio::main]
async fn main() -> Result<(), ToolError> {
    // Initialize logging
    env_logger::init();
    
    let cli = Cli::parse();
    cli.execute().await
}