use devbox_cli::cli::{Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> devbox_cli::error::Result<()> {
    env_logger::init();
    let _cli = Cli::parse();
    
    // match cli.command {
    //     Commands::Start(args) => args.handle().await?,
    // } 
    Ok(())
}