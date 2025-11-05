use clap::Args;
use std::io::{self, Write};
use crate::error::Result;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Project name
    pub name: Option<String>,
    
    /// Skip interactive prompts and use defaults
    #[arg(long)]
    pub yes: bool,
    
    /// Template to use (web, api, fullstack, docker)
    #[arg(long)]
    pub template: Option<String>,
    
    /// Initialize with Docker support
    #[arg(long)]
    pub docker: bool,
}

impl InitArgs {
    pub async fn execute(&self) -> Result<()> {
        println!("Initializing new Devbox project...");
        
        let project_name = self.get_project_name().await?;
        let template = self.select_template().await?;
        let services = self.select_services().await?;
        let with_docker = self.should_include_docker().await?;
        
        self.create_project_structure(&project_name, &template, &services, with_docker).await?;
        self.generate_devbox_yaml(&project_name, &template, &services, with_docker).await?;
        
        if with_docker {
            self.generate_docker_files(&project_name).await?;
        }
        
        println!("Successfully created project: {}", project_name);
        println!("Project location: ./{}", project_name);
        println!("Get started with: cd {} && devbox start", project_name);
        
        Ok(())
    }
    
    async fn get_project_name(&self) -> Result<String> {
        if let Some(name) = &self.name {
            return Ok(name.clone());
        }
        
        if self.yes {
            return Ok("my-devbox-project".to_string());
        }
        
        print!("Project name: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let name = input.trim();
        if name.is_empty() {
            return Ok("my-devbox-project".to_string());
        }
        
        Ok(name.to_string())
    }
    
    async fn select_template(&self) -> Result<String> {
        if let Some(template) = &self.template {
            return Ok(template.clone());
        }
        
        if self.yes {
            return Ok("web".to_string());
        }
        
        println!("\n📋 Select project template:");
        println!("1. Web (Frontend + API)");
        println!("2. API (Backend only)");
        println!("3. Fullstack (Frontend + API + Database)");
        println!("4. Microservices (Multiple services)");
        println!("5. Custom (Choose individual services)");
        
        print!("Choose template [1-5]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => Ok("web".to_string()),
            "2" => Ok("api".to_string()),
            "3" => Ok("fullstack".to_string()),
            "4" => Ok("microservices".to_string()),
            "5" => Ok("custom".to_string()),
            _ => Ok("web".to_string()),
        }
    }
    
    async fn select_services(&self) -> Result<Vec<String>> {
        if self.yes {
            return Ok(vec!["frontend".to_string(), "api".to_string()]);
        }
        
        let template = self.select_template().await?;
        match template.as_str() {
            "web" => Ok(vec!["frontend".to_string(), "api".to_string()]),
            "api" => Ok(vec!["api".to_string()]),
            "fullstack" => Ok(vec!["frontend".to_string(), "api".to_string(), "database".to_string()]),
            "microservices" => Ok(vec!["frontend".to_string(), "api".to_string(), "auth".to_string(), "database".to_string()]),
            "custom" => self.select_custom_services().await,
            _ => Ok(vec!["frontend".to_string(), "api".to_string()]),
        }
    }
    
    async fn select_custom_services(&self) -> Result<Vec<String>> {
        println!("\n  Select services to include:");
        let services = ["frontend", "api", "database", "cache", "auth", 
            "queue", "storage", "monitoring"];
        
        for (i, service) in services.iter().enumerate() {
            println!("{}. {}", i + 1, service);
        }
        
        print!("Select services (comma-separated, e.g., 1,2,3): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let selected: Vec<String> = input
            .trim()
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .filter_map(|i| services.get(i - 1))
            .map(|s| s.to_string())
            .collect();
        
        if selected.is_empty() {
            Ok(vec!["frontend".to_string(), "api".to_string()])
        } else {
            Ok(selected)
        }
    }
    
    async fn should_include_docker(&self) -> Result<bool> {
        if self.docker {
            return Ok(true);
        }
        
        if self.yes {
            return Ok(false);
        }
        
        print!("Include Docker support? [y/N]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes"))
    }
    
    async fn create_project_structure(&self, project_name: &str, _template: &str, services: &[String], with_docker: bool) -> Result<()> {
        println!("Creating project structure...");
        
        // Create project directory
        std::fs::create_dir_all(project_name)?;
        
        // Create service directories
        for service in services {
            let service_dir = format!("{}/{}", project_name, service);
            std::fs::create_dir_all(&service_dir)?;
            
            // Create basic files for each service type
            match service.as_str() {
                "frontend" => {
                    std::fs::write(format!("{}/package.json", service_dir), FRONTEND_PACKAGE_JSON)?;
                    std::fs::write(format!("{}/index.html", service_dir), FRONTEND_HTML)?;
                }
                "api" => {
                    std::fs::write(format!("{}/package.json", service_dir), API_PACKAGE_JSON)?;
                    std::fs::write(format!("{}/server.js", service_dir), API_SERVER_JS)?;
                }
                "database" => {
                    std::fs::write(format!("{}/init.sql", service_dir), DATABASE_INIT_SQL)?;
                }
                _ => {
                    // Create empty directory for other services
                    std::fs::write(format!("{}/README.md", service_dir), format!("# {}\n\nService configuration.", service))?;
                }
            }
        }
        
        if with_docker {
            std::fs::create_dir_all(format!("{}/docker", project_name))?;
        }
        
        Ok(())
    }
    
    async fn generate_devbox_yaml(&self, project_name: &str, template: &str, services: &[String], with_docker: bool) -> Result<()> {
        println!("Generating devbox.yaml...");
        
        let mut yaml_content = format!(
            "name: \"{}\"\ndescription: \"{} project\"\n\n",
            project_name, template
        );
        
        yaml_content.push_str("commands:\n  start:\n    dev: \"echo 'Starting development environment'\"\n    build: \"echo 'Building project'\"\n    test: \"echo 'Running tests'\"\n\n");
        
        yaml_content.push_str("services:\n");
        for service in services {
            let service_config = match service.as_str() {
                "frontend" => FRONTEND_SERVICE_CONFIG,
                "api" => API_SERVICE_CONFIG,
                "database" => DATABASE_SERVICE_CONFIG,
                "cache" => CACHE_SERVICE_CONFIG,
                "auth" => AUTH_SERVICE_CONFIG,
                _ => GENERIC_SERVICE_CONFIG,
            };
            yaml_content.push_str(service_config);
            yaml_content.push('\n');
        }
        
        if with_docker {
            yaml_content.push_str("\nenvironment:\n  DOCKER_ENABLED: \"true\"\n");
        }
        
        yaml_content.push_str("\nhooks:\n  pre_start: \"echo 'Setting up development environment'\"\n  post_start: \"echo 'All services are ready!'\"\n");
        
        std::fs::write(format!("{}/devbox.yaml", project_name), yaml_content)?;
        Ok(())
    }
    
    async fn generate_docker_files(&self, project_name: &str) -> Result<()> {
        println!("Generating Docker files...");
        
        // Dockerfile for frontend
        std::fs::write(
            format!("{}/docker/Dockerfile.frontend", project_name),
            DOCKERFILE_FRONTEND
        )?;
        
        // Dockerfile for API
        std::fs::write(
            format!("{}/docker/Dockerfile.api", project_name),
            DOCKERFILE_API
        )?;
        
        // docker-compose.yml
        std::fs::write(
            format!("{}/docker-compose.yml", project_name),
            DOCKER_COMPOSE
        )?;
        
        // .dockerignore
        std::fs::write(
            format!("{}/.dockerignore", project_name),
            DOCKER_IGNORE
        )?;
        
        Ok(())
    }
}

// Template content constants
const FRONTEND_PACKAGE_JSON: &str = r#"{
  "name": "frontend",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }
}"#;

const FRONTEND_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Devbox Frontend</title>
</head>
<body>
    <h1>Welcome to Devbox!</h1>
    <p>Your frontend service is running.</p>
</body>
</html>"#;

const API_PACKAGE_JSON: &str = r#"{
  "name": "api",
  "version": "1.0.0",
  "scripts": {
    "dev": "node server.js",
    "start": "node server.js"
  }
}"#;

const API_SERVER_JS: &str = r#"const http = require('http');

const server = http.createServer((req, res) => {
  res.writeHead(200, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ message: 'Hello from Devbox API!' }));
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
  console.log(`API server running on port ${PORT}`);
});"#;

const DATABASE_INIT_SQL: &str = "-- Database initialization script\nCREATE TABLE IF NOT EXISTS users (\n    id SERIAL PRIMARY KEY,\n    name VARCHAR(100),\n    email VARCHAR(100)\n);";

// Service configurations for devbox.yaml
const FRONTEND_SERVICE_CONFIG: &str = "  - name: \"frontend\"\n    service_type: \"web\"\n    command: \"cd frontend && npm run dev\"\n    working_dir: \"./frontend\"\n    health_check:\n      type_entry: \"http\"\n      port: 5173\n      http_target: \"http://localhost:5173\"\n    dependencies: []";

const API_SERVICE_CONFIG: &str = "  - name: \"api\"\n    service_type: \"api\"\n    command: \"cd api && npm run dev\"\n    working_dir: \"./api\"\n    health_check:\n      type_entry: \"http\"\n      port: 3000\n      http_target: \"http://localhost:3000\"\n    dependencies: []";

const DATABASE_SERVICE_CONFIG: &str = "  - name: \"database\"\n    service_type: \"database\"\n    command: \"docker run -p 5432:5432 -e POSTGRES_PASSWORD=devbox postgres:15\"\n    health_check:\n      type_entry: \"port\"\n      port: 5432\n      http_target: \"\"\n    dependencies: []";

const CACHE_SERVICE_CONFIG: &str = "  - name: \"cache\"\n    service_type: \"cache\"\n    command: \"docker run -p 6379:6379 redis:7-alpine\"\n    health_check:\n      type_entry: \"port\"\n      port: 6379\n      http_target: \"\"\n    dependencies: []";

const AUTH_SERVICE_CONFIG: &str = "  - name: \"auth\"\n    service_type: \"api\"\n    command: \"echo 'Auth service starting'\"\n    dependencies: [\"database\"]";

const GENERIC_SERVICE_CONFIG: &str = "  - name: \"{}\"\n    service_type: \"service\"\n    command: \"echo 'Service {0} starting'\"\n    dependencies: []";

// Docker templates
const DOCKERFILE_FRONTEND: &str = r#"FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 5173
CMD ["npm", "run", "dev"]"#;

const DOCKERFILE_API: &str = r#"FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 3000
CMD ["npm", "start"]"#;

const DOCKER_COMPOSE: &str = r#"version: '3.8'
services:
  frontend:
    build:
      context: .
      dockerfile: docker/Dockerfile.frontend
    ports:
      - "5173:5173"
    volumes:
      - ./frontend:/app
      - /app/node_modules

  api:
    build:
      context: .
      dockerfile: docker/Dockerfile.api
    ports:
      - "3000:3000"
    volumes:
      - ./api:/app
      - /app/node_modules

  database:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: devbox
      POSTGRES_DB: devbox
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:"#;

const DOCKER_IGNORE: &str = r#"node_modules
npm-debug.log
.git
.env"#;