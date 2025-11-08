use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub commands: Commands,
    pub services: Option<Vec<Service>>,
    pub environment: Option<HashMap<String, String>>,
    pub hooks: Option<Hooks>,

    #[serde(skip)]  // Don't serialize/deserialize this from YAML
    pub base_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Commands {
    pub start: StartCommands
    // add other later
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StartCommands {
    pub dev: String,
    pub test: Option<String>,
    pub build: String,
    pub clean: Option<String>,

    pub services: Option<Services>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Services {
    pub services: Vec<Service>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub name: String,
    pub service_type: String,
    pub command: String,
    pub working_dir: Option<String>,
    pub health_check: Option<HealthCheck>,
    pub dependencies: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheck {
    pub type_entry: String,
    pub port: Option<i16>,
    pub http_target: String
} 

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hooks {
    pub pre_start: Option<String>,
    pub post_start: Option<String>,
    pub pre_stop: Option<String>,
    pub post_stop: Option<String>
}

impl ProjectConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {  // FIXED: Add error type
        let content = std::fs::read_to_string(path)?;
        let mut config: ProjectConfig = serde_yaml::from_str(&content)?;
        
        // Store the config file directory as base path
        config.base_path = Some(
            std::path::Path::new(path)
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::path::Path::new(".").to_path_buf())
        );
            
        Ok(config)
    }
    
    pub fn resolve_path(&self, relative_path: &str) -> PathBuf {
        if let Some(base_path) = &self.base_path {
            base_path.join(relative_path)
        } else {
            // Fallback if base_path is not set (shouldn't happen with from_file)
            PathBuf::from(relative_path)
        }
    }
}