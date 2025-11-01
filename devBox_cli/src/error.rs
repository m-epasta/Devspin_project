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

}

pub type Result<T> = std::result::Result<T, ToolError>;