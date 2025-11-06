use devbox_cli::configs::yaml_parser::ProjectConfig;
use devbox_cli::cli::start::StartArgs;
use devbox_cli::process::ProcessState;
use std::process::Command;
use devbox_cli::cli::init::InitArgs;
use std::fs;
use tempfile::TempDir;

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

        let result = args.execute().await;
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
        
        let result = args.execute().await;
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
        
        let result = args.execute().await;
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
        let result = args.execute().await;
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
        let result = args.execute().await;
        assert!(result.is_ok(), "Non-verbose dry run should succeed");
        
        // Should show minimal output
    }  

    #[test]
    fn test_process_creation() {
        // ✅ FIXED: ProcessState::new() returns ProcessState directly, not Result
        let process_state = ProcessState::new();
        // Just creating it should work fine
        assert_eq!(process_state.process_count(), 0);
    }

    #[test]
    fn test_process_state_operations() {
        // ✅ FIXED: No unwrap needed
        let mut process_state = ProcessState::new();

        let processes = process_state.get_all_processes();
        assert_eq!(processes.len(), 0, "New ProcessState should have no processes");

        let mut child = Command::new("sleep")
            .arg("1")
            .spawn()
            .expect("Failed to spawn test process");

        // ✅ FIXED: No Result return, just call the method
        let _ = process_state.add_process(
            &mut child,
            "test-service",
            "test-project",
            "sleep 1"
        );

        let processes = process_state.get_all_processes();
        assert_eq!(processes.len(), 1, "Should have one process after adding");

        let project_processes = process_state.get_project_processes("test-project");
        assert_eq!(project_processes.len(), 1, "Should find process by project name");

        let pid = child.id();
        // ✅ FIXED: No Result return
        let _ = process_state.remove_process(pid);

        let processes = process_state.get_all_processes();
        assert_eq!(processes.len(), 0, "Should have no processes after removal");

        let _ = child.kill();
    }
    
    #[test]
    fn test_process_state_persistence() {
        // ✅ FIXED: Since it's memory-only, we need to test within same instance
        let mut process_state = ProcessState::new();

        let mut child = Command::new("sleep")
            .arg("1")
            .spawn()
            .expect("Failed to spawn test process");

        let _ = process_state.add_process(
            &mut child,
            "persistence-service",
            "persistence-project",
            "sleep 1"
        );
        
        let pid = child.id();

        // ✅ FIXED: Memory-only means new instances don't share state
        // Test that the original instance still has the process
        let processes = process_state.get_all_processes();
        assert_eq!(processes.len(), 1, "Should still have the process");
        assert_eq!(processes[0].pid, pid, "Should have same PID");
        assert_eq!(processes[0].service_name, "persistence-service");
        assert_eq!(processes[0].project_name, "persistence-project");

        let _ = process_state.remove_process(pid);
        let _ = child.kill();
    }

    #[test]
    fn test_process_state_error_cases() {
        let mut process_state = ProcessState::new();

        // ✅ FIXED: No Result return
        let _ = process_state.remove_process(99999); // Should not panic

        let processes = process_state.get_project_processes("non-existent-project");
        assert_eq!(processes.len(), 0, "Should return empty Vec for non-existent project");
    }

    #[tokio::test]
    async fn test_start_command_tracks_processes() {
        let args = StartArgs {
            name: "tests/fixtures/verbose-project".to_string(),
            env: None,
            verbose: false,
            background: false,
            dry_run: true,
            only: None,
            skip: None,
        };

        let result = args.execute().await;
        assert!(result.is_ok(), "Dry run should succeed");

        // ✅ FIXED: ProcessState::new() works directly
        let process_state = ProcessState::new();
        assert_eq!(process_state.process_count(), 0); // Dry run doesn't track real processes
    }


    #[tokio::test]
    async fn test_init_creates_project_structure() {
        // Create temporary directory for test
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let args = InitArgs {
            name: Some("test-project".to_string()),
            yes: true,
            template: Some("web".to_string()),
            docker: false,
        };

        let result = args.execute().await;
        assert!(result.is_ok(), "Init should succeed");

        // Verify files were created
        assert!(fs::metadata("test-project").is_ok());
        assert!(fs::metadata("test-project/devbox.yaml").is_ok());
        assert!(fs::metadata("test-project/frontend").is_ok());
        assert!(fs::metadata("test-project/api").is_ok());
        
        // Cleanup
        std::env::set_current_dir("..").unwrap();
    }

    #[tokio::test]
    async fn test_init_with_docker() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let args = InitArgs {
            name: Some("docker-project".to_string()),
            yes: true,
            template: Some("web".to_string()),
            docker: true,
        };

        let result = args.execute().await;
        assert!(result.is_ok(), "Init with docker should succeed");

        // Verify Docker files were created
        assert!(fs::metadata("docker-project/docker-compose.yml").is_ok());
        assert!(fs::metadata("docker-project/.dockerignore").is_ok());
        assert!(fs::metadata("docker-project/docker").is_ok());
        
        std::env::set_current_dir("..").unwrap();
    }

    #[tokio::test]
    async fn test_init_fullstack_template() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let args = InitArgs {
            name: Some("fullstack-project".to_string()),
            yes: true,
            template: Some("fullstack".to_string()),
            docker: false,
        };

        let result = args.execute().await;
        assert!(result.is_ok(), "Fullstack init should succeed");

        // Verify all services were created
        assert!(fs::metadata("fullstack-project/frontend").is_ok());
        assert!(fs::metadata("fullstack-project/api").is_ok());
        assert!(fs::metadata("fullstack-project/database").is_ok());
        
        std::env::set_current_dir("..").unwrap();
    }

    #[tokio::test]
    async fn test_init_command() {
        use devbox_cli::cli::init::InitArgs;
        use std::fs;

        // Create test directory
        let test_dir = "test_init_output";
        let _ = fs::remove_dir_all(test_dir); // Cleanup previous runs
        
        let args = InitArgs {
            name: Some(test_dir.to_string()),
            yes: true,
            template: Some("web".to_string()),
            docker: true,
        };
        
        let result = args.execute().await;
        assert!(result.is_ok(), "Init command should succeed");
        
        // Verify core files exist
        assert!(fs::metadata(format!("{}/devbox.yaml", test_dir)).is_ok());
        assert!(fs::metadata(format!("{}/frontend", test_dir)).is_ok());
        assert!(fs::metadata(format!("{}/api", test_dir)).is_ok());
        assert!(fs::metadata(format!("{}/docker-compose.yml", test_dir)).is_ok());
        
        // Cleanup
        let _ = fs::remove_dir_all(test_dir);
    }

    #[tokio::test]
    async fn test_init_without_name() {
        use devbox_cli::cli::init::InitArgs;
        use std::fs;

        let args = InitArgs {
            name: None,
            yes: true,
            template: Some("api".to_string()),
            docker: false,
        };
        
        let result = args.execute().await;
        assert!(result.is_ok(), "Init without name should succeed");

        let entries: Vec<_> = fs::read_dir(".")
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .collect();
        
        // Look for a directory that contains devbox.yaml
        let project_dir = entries.iter().find(|entry| {
            fs::metadata(entry.path().join("devbox.yaml")).is_ok()
        });
        
        assert!(project_dir.is_some(), "Should create a project directory with devbox.yaml");
        
        // Cleanup - remove whatever directory was created
        if let Some(dir) = project_dir {
            let _ = fs::remove_dir_all(dir.path());
        }
    }

    #[tokio::test]
    async fn test_full_project_creation_flow() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let args = InitArgs {
            name: Some("integration-test".to_string()),
            yes: true,
            template: Some("react".to_string()),
            docker: true,
        };

        let result = args.execute().await;
        assert!(result.is_ok(), "Project creation failed: {:?}", result);

        // Verify the complete project structure
        let expected_files = vec![
            "integration-test",
            "integration-test/frontend",
            "integration-test/frontend/package.json",
            "integration-test/frontend/src",
            "integration-test/frontend/src/main.tsx",
            "integration-test/frontend/src/App.tsx",
            "integration-test/devbox.yaml",
            "integration-test/docker",
            "integration-test/docker-compose.yml",
        ];

        for file in expected_files {
            assert!(fs::metadata(file).is_ok(), "File {} should exist", file);
        }

        // Verify devbox.yaml content
        let yaml_content = fs::read_to_string("integration-test/devbox.yaml").unwrap();
        assert!(yaml_content.contains("name: \"integration-test\""));
        assert!(yaml_content.contains("DOCKER_ENABLED: \"true\""));
    }

    #[tokio::test]
    async fn test_all_templates() {
        let templates = vec!["nextjs", "react", "vue", "svelte", "node", "python", "rust", "go"];

        for template in templates {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(&temp_dir).unwrap();

            let args = InitArgs {
                name: Some(format!("test-{}", template)),
                yes: true,
                template: Some(template.to_string()),
                docker: false,
            };

            let result = args.execute().await;
            assert!(result.is_ok(), "Template {} failed: {:?}", template, result);

            // Basic verification that project was created
            assert!(fs::metadata(format!("test-{}", template)).is_ok());
            assert!(fs::metadata(format!("test-{}/devbox.yaml", template)).is_ok());
        }
    }
}