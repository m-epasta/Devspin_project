use devbox_cli::configs::yaml_parser::ProjectConfig;
use devbox_cli::cli::start::StartArgs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_valid_config() {
        let yaml_content = r#"
        name: "test-app"
        description: "test: should succeed"
        commands:
            start:
                dev: "npm run dev"        
                build: "npm run build"    
        "#;
        let config: ProjectConfig = serde_yaml::from_str(yaml_content).unwrap();

        assert_eq!(config.name, "test-app");
        assert_eq!(config.commands.start.dev, "npm run dev")
    }

    #[test]
    fn test_invalid_config_fails() {
        let invalid_yaml = "name: 123";

        let result = serde_yaml::from_str::<ProjectConfig>(invalid_yaml);
        assert!(result.is_err());
        // TODO: try more errors that is possible
    }

    #[tokio::test]
    async fn test_start_command_dry_run() {
        let args = StartArgs {
            name: "tests/fixtures/commands_test.rs".to_string(),  
            env: None,
            verbose: false,
            background: false,
            dry_run: true,
            only: None,
            skip: None,
        };
        
        println!("Testing with project name: {}", args.name);
        println!("Looking for file: {}/devbox.yaml", args.name);

        let result = args.handle().await;
        assert!(result.is_ok(), "Dry run should succeed");
    }

    #[tokio::test]
    async fn test_start_command_with_filters() {
        let args = StartArgs {
            name: "tests/fixtures/test-project".to_string(), 
            env: None,
            verbose: true,
            background: false,
            dry_run: true,
            only: Some(vec!["frontend".to_string()]),
            skip: Some(vec!["database".to_string()]),
        };
        
        let result = args.handle().await;
        assert!(result.is_ok(), "Filtered dry run should succeed");
    }

    #[tokio::test]
    async fn test_start_background_command() {
        let args = StartArgs {
            name: "tests/fixtures/background-project".to_string(),  
            env: None,
            verbose: true,
            background: true,  
            dry_run: true,     
            only: Some(vec!["frontend".to_string()]),
            skip: Some(vec!["database".to_string()]),
        };
        
        let result = args.handle().await;
        assert!(result.is_ok(), "Background dry run should succeed");
    }

    #[tokio::test]
    async fn test_start_verbose_command() {
        let args = StartArgs {
            name: "tests/fixtures/verbose-project".to_string(),
            env: None,
            verbose: true,  
            background: false,
            dry_run: true,
            only: None,
            skip: None,
        };
        
        println!("Testing VERBOSE mode...");
        let result = args.handle().await;
        assert!(result.is_ok(), "Verbose dry run should succeed");
        
        // The test should show extra details in output due to verbose mode
    } 
    
    #[tokio::test]
    async fn test_start_non_verbose_command() {
        let args = StartArgs {
            name: "tests/fixtures/verbose-project".to_string(), 
            env: None,
            verbose: false,  // ← No verbose
            background: false,
            dry_run: true,
            only: None,
            skip: None,
        };
        
        println!("Testing NON-VERBOSE mode...");
        let result = args.handle().await;
        assert!(result.is_ok(), "Non-verbose dry run should succeed");
        
        // Should show minimal output
    }  

}
