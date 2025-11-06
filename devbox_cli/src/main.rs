use clap::Parser;
use devbox_cli::cli::Cli;
use devbox_cli::error::ToolError;

#[tokio::main]
async fn main() -> Result<(), ToolError> {
    // Initialize logging
    env_logger::init();
    
    let cli = Cli::parse();
    cli.execute().await
}