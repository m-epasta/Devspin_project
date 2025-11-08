use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolError {

    #[error("Project not found {0}")]
    ProjectNotFound(String),

    #[error("Config error happened {0}")]
    ConfigError(String),

    #[error("Error happened during the process {0}")]
    ProcessError(String),   

    #[error("Network error happened {0}")]
    NetworkError(String),

    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("Failed to parse YAML: {0}")]
    ParseError(String),
    
    #[error("Config validation failed: {0}")]
    ValidationError(String),
        
    #[error("Generic error: {0}")]
    GenericError(String),

}

impl From<Box<dyn std::error::Error>> for ToolError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ToolError::GenericError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ToolError>;