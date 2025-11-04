    use std::collections::HashMap;

    use clap::Args;
    use crate::error::{Result, ToolError};
    use  crate::configs::yaml_parser::{ProjectConfig, Service};
    use tokio::process::Command;
    use log::{info, debug};

    #[derive(Debug, Args, Clone)]
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
        pub async fn handle(&self) -> Result<()> {
            info!("Starting project: {}", self.name);

            let default_path = format!("{}/devbox.yaml", self.name);
            let project = self.load_project(&default_path).await?;

            if self.dry_run {
                debug!("dry running");
                return self.dry_run(&project);
            }

            if let Some(env) = &self.env {
                info!("Starting with env file: {}", env);
                self.load_env_file(env).await?;
            }

            if self.verbose {
                info!("Verbose output enabled")
            }

            // FIX: Make background and foreground modes exclusive
            if self.background {
                info!("Running in background");
                return self.start_in_background(project).await;  // ← This returns, no duplicate start
            }

            if let Some(only_services) = &self.only {
                info!("Starting only: {:?}", only_services)
            }

            if let Some(skip_services) = &self.skip {
                info!("Starting without: {:?}", skip_services)
            }

            // This only runs if NOT in background mode
            self.start_services(&project).await
        }

        async fn load_project(&self, path: &str) -> Result<ProjectConfig> {
            debug!("loading the project {}", self.name);
            let project = ProjectConfig::from_file(path)?;
            info!("successfully loaded the project: {}", project.name);

            Ok(project)
        }

        async fn load_env_file(&self, env_file: &str) -> Result<()> {
            dotenvy::from_filename(env_file)
                .map_err(|e| ToolError::ConfigError(format!("Failed to load env file {}: {}", env_file, e)))?;

            info!("Loaded environment variables from: {}", env_file);
            Ok(())
        }


    pub fn dry_run(&self, project: &ProjectConfig) -> Result<()> {
        println!("🚀 DRY RUN - Would start project: {}", project.name);

        if self.verbose {
            println!("📁 CONFIGURATION DETAILS:");
            println!("   Config path: ./{}/devbox.yaml", self.name);
            println!("   Project: {}", project.name);
            println!("   Description: {:?}", project.description);
            
            if let Some(env) = &self.env {
                println!("   Environment file: {}", env);
            }
            
            println!("   Service filters: only={:?}, skip={:?}", self.only, self.skip);
            
            // Show commands
            println!("   Commands:");
            println!("     - Dev: {}", project.commands.start.dev);
            if let Some(test) = &project.commands.start.test {
                println!("     - Test: {}", test);
            }
                println!("     - Build: {}", project.commands.start.build);

            if let Some(clean) = &project.commands.start.clean {
                println!("     - Clean: {}", clean);
            }
            
            // Show environment variables
            if let Some(env_vars) = &project.environment {
                println!("   Environment variables ({}):", env_vars.len());
                for (key, value) in env_vars {
                    println!("     - {}={}", key, value);
                }
            }
            
            // Show hooks
            if let Some(hooks) = &project.hooks {
                println!("   Hooks:");
                if let Some(pre_start) = &hooks.pre_start {
                    println!("     - Pre-start: {}", pre_start);
                }
                if let Some(post_start) = &hooks.post_start {
                    println!("     - Post-start: {}", post_start);
                }
                if let Some(pre_stop) = &hooks.pre_stop {
                    println!("     - Pre-stop: {}", pre_stop);
                }
                if let Some(post_stop) = &hooks.post_stop {
                    println!("     - Post-stop: {}", post_stop);
                }
            }
            
            println!("\n");
        }

        if self.background {
            println!("📡 Mode: Background (detached)");
        } else {
            println!("👀 Mode: Foreground (attached)");
        }
        
        if let Some(services) = &project.services {
            println!("\n");
            println!("🛠️ SERVICES:");
            for service in services {  // ← Direct iteration, no .services
                let should_start = self.should_start_service(service);
                let status = if should_start { "✅" } else { "❌" };
                
                if self.verbose {
                    println!("  {} {}:", status, service.name);
                    println!("     Type: {}", service.service_type);
                    println!("     Command: {}", service.command);
                    
                    if let Some(dir) = &service.working_dir {
                        println!("     Working directory: {}", dir);
                    }
                    
                    println!("     Dependencies: {:?}", service.dependencies);
                    
                    if let Some(health_check) = &service.health_check {
                        println!("     Health check:");
                        println!("       - Type: {}", health_check.type_entry);
                        if let Some(port) = health_check.port {
                            println!("       - Port: {}", port);
                        }
                        if !health_check.http_target.is_empty() {
                            println!("       - HTTP target: {}", health_check.http_target);
                        }
                    }
                    
                    if !should_start {
                        println!("     Status: SKIPPED (filtered out)");
                    }
                    
                    println!("\n");
                } else {
                    if should_start {
                        println!("  ✅ {}: {}", service.name, service.command);
                    } else {
                        println!("  ❌ {}: (skipped)", service.name);
                    }
                }
            }
            
            if self.verbose {
                println!("---");
                println!("💡 Total services: {}", services.len());  
                println!("🎯 Filters applied: only={:?}, skip={:?}", self.only, self.skip);
            }
        }

        Ok(())     
    }

        fn should_start_service(&self, service: &Service) -> bool {
        if let Some(only_services) = &self.only {
            if !only_services.contains(&service.name) {
                return false
            }
        }

        if let Some(skip_services) = &self.skip {
            if skip_services.contains(&service.name) {
                return false
            }
        }
        true
    }

        async fn run_service_command(&self, service: &Service, env_vars: &HashMap<String, String>) -> Result<()> {
            info!("Starting service: {}", service.name);
            
            let mut command = Command::new("sh");
            command.arg("-c").arg(&service.command);
            
            // Set working directory
            if let Some(working_dir) = &service.working_dir {
                command.current_dir(working_dir);
                debug!("Working directory: {}", working_dir);
            }
            
            // Set environment variables
            for (key, value) in env_vars {
                command.env(key, value);
                debug!("  Env: {}={}", key, value);
            }
            
            // Execute command
            let mut child = command.spawn()
                .map_err(|e| ToolError::ProcessError(format!("Failed to start service {}: {}", service.name, e)))?;
            
            // Wait for completion
            let status = child.wait().await
                .map_err(|e| ToolError::ProcessError(format!("Service {} failed: {}", service.name, e)))?;
            
            if status.success() {
                info!("Service {} started successfully", service.name);
                Ok(())
            } else {
                Err(ToolError::ProcessError(format!(
                    "Service {} exited with code: {:?}",
                    service.name, status.code()
                )))
            }
        }

        async fn start_services(&self, project: &ProjectConfig) -> Result<()> {
            let env_vars = project.environment.clone().unwrap_or_default();
            
            if let Some(services) = &project.services {
                for service in services { 
                    if self.should_start_service(service) {
                        self.run_service_command(service, &env_vars).await?;
                    }
                }
            }
            
            info!("All services started successfully!");
            Ok(())
        }

        async fn start_in_background(&self, project: ProjectConfig) -> Result<()> {
            let args_clone = self.clone();
            
            tokio::spawn(async move {
                if let Err(e) = args_clone.start_services(&project).await {
                    eprintln!("Background service error: {}", e);
                }
            });
            
            Ok(())
        }
    }

