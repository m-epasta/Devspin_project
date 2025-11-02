use clap::Args;
use crate::error::Result;
use  crate::config::yaml_parser::{ProjectConfig, Service};
use log::{error, warn, info, debug, trace};

#[derive(Args)]
pub struct StartArgs {
    /// Project name
    pub name: String,
    
    /// Environment configuration file
    #[arg(long)]
    pub env: Option<String>,
    
    /// Show detailed output
    #[arg(long)]
    pub verbose: bool,

    /// Run in background
    #[arg(long)]
    pub background: bool,

    /// Show what would start without actually starting
    #[arg(long)]
    pub dry_run: bool,

    /// Only start specific services
    #[arg(long, value_delimiter = ',')]
    pub only: Option<Vec<String>>,

    /// Skip specific services
    #[arg(long, value_delimiter = ',')]
    pub skip: Option<Vec<String>>
}

impl StartArgs {
    pub fn handle(&self) -> Result<()> {
        info!("Starting project: {}", self.name);
        info!("Loading project: {}", self.name);

        if let Some(env) = &self.env {
            info!("Starting with an env variable")
        }

        if self.verbose {
            info!("Verbose output enabled")
        }
        
        if self.background {
            info!("Running in background")
        }

        if self.dry_run {
            println!("DRY RUN - would start:");
            // implement logic
            return Ok(())
        }

        if let Some(only_services) = &self.only {
            info!("Starting only : {:?}", only_services)
        }

        if let Some(skip_services) = &self.skip {
            info!("Starting without: {:?}", skip_services)
        }

        Ok(())
    }

    fn load_project(&self, path: &str) -> Result<()> {
        debug!("loading the project {}", self.name);
        let project = ProjectConfig::from_file(&format!("{}/devbox.yaml", self.name))?;
        info!("successfully loaded the project");

        if self.dry_run {
            debug!("dry running");
            return self.dry_run(&project);

        }
        // TODO:
        Ok(())
    }


    fn dry_run(&self, project: &ProjectConfig) -> Result<()> {
        println!("🚀 DRY RUN - Would start project: {}", project.name);
        
        if let Some(services) = &project.commands.start.services {
            println!("Services:");
            for service in &services.services {
                if self.should_start_service(service) {
                    debug!("  ✅ {}: {}", service.name, service.command);
                } else {
                    debug!("  ❌ {}: (skipped)", service.name);
                }
            }
        }
        
        Ok(())     
    }

    fn should_start_service(&self, service: &Service) -> bool {
    // Add your --only/--skip logic here
    true
}

}

