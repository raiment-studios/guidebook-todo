use anyhow::{Context, Result};
use dirs::home_dir;
use reqwest::Client;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::io::{self, Write};

pub async fn run_init() -> Result<()> {
    let data_dir = get_data_dir()?;
    
    if data_dir.exists() {
        println!("guidebook-todo is already initialized.");
        return Ok(());
    }
    
    println!("Welcome to guidebook-todo!");
    println!("Setting up your TODO management system...");
    println!();
    
    // Check for local TODO files
    if let Ok(local_file) = find_local_todo_file() {
        println!("Found existing TODO file: {}", local_file.display());
        println!("You can continue using this file, or set up the global guidebook directory.");
        println!();
    }
    
    println!("guidebook-todo stores its data locally with optional GitHub backup.");
    println!("If you already have a 'guidebook-data' repository on GitHub, you can");
    println!("link it for automatic synchronization.");
    println!();
    println!("Would you like to:");
    println!("1. Create a new guidebook-data repository on GitHub");
    println!("2. Skip GitHub integration (local-only mode)");
    println!("3. Exit without making changes");
    println!();
    
    print!("Enter your choice (1-3): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    match choice {
        "1" => {
            setup_with_github(&data_dir).await?;
        }
        "2" => {
            setup_local_only(&data_dir)?;
        }
        "3" => {
            println!("Setup cancelled.");
            return Ok(());
        }
        _ => {
            anyhow::bail!("Invalid choice. Please run 'todo init' again.");
        }
    }
    
    println!("✓ guidebook-todo is ready to use!");
    Ok(())
}

fn get_data_dir() -> Result<PathBuf> {
    let home = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    
    Ok(home.join(".local").join("share").join("guidebook"))
}

fn find_local_todo_file() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let local_files = ["TODO.yaml", "TODO.yml", "todo.yaml", "todo.yml"];
    
    for filename in &local_files {
        let path = current_dir.join(filename);
        if path.exists() {
            return Ok(path);
        }
    }
    
    anyhow::bail!("No local TODO file found")
}

async fn setup_with_github(data_dir: &PathBuf) -> Result<()> {
    println!("Setting up GitHub integration...");
    
    let access_token = github_oauth().await?;
    create_github_repository(&access_token).await?;
    
    // Create local directory structure
    std::fs::create_dir_all(&data_dir.join("guidebook-todo"))?;
    
    // Initialize git repository
    init_git_repository(data_dir)?;
    
    // Create default TODO file
    create_default_todo_file(data_dir)?;
    
    println!("✓ GitHub repository created and linked");
    Ok(())
}

fn setup_local_only(data_dir: &PathBuf) -> Result<()> {
    println!("Setting up local-only mode...");
    
    // Create local directory structure
    std::fs::create_dir_all(&data_dir.join("guidebook-todo"))?;
    
    // Initialize git repository (local only)
    init_git_repository(data_dir)?;
    
    // Create default TODO file
    create_default_todo_file(data_dir)?;
    
    println!("✓ Local setup complete");
    Ok(())
}

async fn github_oauth() -> Result<String> {
    let client_id = "your-github-client-id"; // TODO: Replace with actual client ID
    let client = Client::new();
    
    // 1. Request device code
    let device_response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", client_id), ("scope", "repo")])
        .send()
        .await
        .context("Failed to request device code")?;
    
    let device_data: Value = device_response.json().await?;
    let verification_uri = device_data["verification_uri"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing verification_uri"))?;
    let user_code = device_data["user_code"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing user_code"))?;
    
    // 2. Show user instructions
    println!("To authorize guidebook-todo, visit: {}", verification_uri);
    println!("And enter the code: {}", user_code);
    println!("Waiting for authorization...");
    
    // 3. Poll for token
    let access_token = poll_for_token(&client, client_id, &device_data).await?;
    
    Ok(access_token)
}

async fn poll_for_token(client: &Client, client_id: &str, device_data: &Value) -> Result<String> {
    let device_code = device_data["device_code"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing device_code"))?;
    let interval = device_data["interval"]
        .as_u64()
        .unwrap_or(5);
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        
        let token_response = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", client_id),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?;
        
        let token_data: Value = token_response.json().await?;
        
        if let Some(access_token) = token_data["access_token"].as_str() {
            return Ok(access_token.to_string());
        }
        
        if let Some(error) = token_data["error"].as_str() {
            match error {
                "authorization_pending" => continue,
                "slow_down" => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
                _ => anyhow::bail!("OAuth error: {}", error),
            }
        }
    }
}

async fn create_github_repository(access_token: &str) -> Result<()> {
    let client = Client::new();
    
    let repo_response = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "guidebook-todo")
        .json(&json!({
            "name": "guidebook-data",
            "description": "Personal guidebook data repository",
            "private": false,
            "auto_init": true
        }))
        .send()
        .await
        .context("Failed to create GitHub repository")?;
    
    if !repo_response.status().is_success() {
        let error_text = repo_response.text().await?;
        anyhow::bail!("Failed to create repository: {}", error_text);
    }
    
    Ok(())
}

fn init_git_repository(data_dir: &PathBuf) -> Result<()> {
    use std::process::Command;
    
    // Initialize git repository
    let output = Command::new("git")
        .args(&["init"])
        .current_dir(data_dir)
        .output()
        .context("Failed to initialize git repository")?;
    
    if !output.status.success() {
        anyhow::bail!("Git init failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

fn create_default_todo_file(data_dir: &PathBuf) -> Result<()> {
    let todo_file = data_dir.join("guidebook-todo").join("todo.yaml");
    
    let default_content = r#"next_id: 1
todos: []
"#;
    
    std::fs::write(&todo_file, default_content)
        .with_context(|| format!("Failed to create TODO file: {}", todo_file.display()))?;
    
    Ok(())
}
