use anyhow::Result;
use colored::*;
use dialoguer::Input;
use serde_json::json;
use std::path::Path;
use crate::config::Config;
use crate::main::AdminAction;

pub async fn execute(action: AdminAction) -> Result<()> {
    let config = Config::load()?;
    
    if config.auth_token.is_none() {
        println!("{}", "Error: Admin authentication required".red());
        println!("Please run 'cpkgs auth login' first");
        return Ok(());
    }

    match action {
        AdminAction::Upload { package_file } => {
            if !Path::new(&package_file).exists() {
                println!("{}: Package file not found", "Error".red());
                return Ok(());
            }
            
            println!("{} {}", "Uploading".blue().bold(), package_file.cyan());
            
            let client = reqwest::Client::new();
            let url = format!("{}/api/packages", config.registry_url);
            
            let file_content = tokio::fs::read(&package_file).await?;
            let file_name = Path::new(&package_file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("package.deb");
            
            let form = reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::bytes(file_content)
                    .file_name(file_name.to_string())
                    .mime_str("application/octet-stream")?);
            
            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", config.auth_token.unwrap()))
                .multipart(form)
                .send()
                .await?;

            if response.status().is_success() {
                let result: serde_json::Value = response.json().await?;
                println!("{} {} successfully", "✓ Uploaded".green(), result["name"]);
            } else {
                println!("{}: {}", "Upload failed".red(), response.text().await?);
            }
        }
        
        AdminAction::Remove { name, version } => {
            println!("{} {} ({})", "Removing".red().bold(), name.cyan(), version.yellow());
            
            let client = reqwest::Client::new();
            let url = format!("{}/api/packages/{}/{}", config.registry_url, name, version);
            
            let response = client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", config.auth_token.unwrap()))
                .send()
                .await?;

            if response.status().is_success() {
                println!("{} {} ({})", "✓ Removed".green(), name.cyan(), version.yellow());
            } else {
                println!("{}: {}", "Removal failed".red(), response.text().await?);
            }
        }
        
        AdminAction::ListUsers => {
            let client = reqwest::Client::new();
            let url = format!("{}/api/users", config.registry_url);
            
            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", config.auth_token.unwrap()))
                .send()
                .await?;

            if response.status().is_success() {
                let users: serde_json::Value = response.json().await?;
                
                println!("{}", "Users:".green().bold());
                println!("{}", "─".repeat(50));
                
                if let Some(users_array) = users.as_array() {
                    for user in users_array {
                        println!("{} ({})", user["username"], user["email"]);
                        println!("  Role: {}", user["role"]);
                        println!("  Created: {}", user["created_at"]);
                        println!();
                    }
                }
            } else {
                println!("{}: {}", "Error".red(), response.text().await?);
            }
        }
        
        AdminAction::CreateUser { username, email } => {
            let password: String = Input::new()
                .with_prompt("Password for new user")
                .interact()?;
            
            let client = reqwest::Client::new();
            let url = format!("{}/api/users", config.registry_url);
            
            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", config.auth_token.unwrap()))
                .json(&json!({
                    "username": username,
                    "email": email,
                    "password": password
                }))
                .send()
                .await?;

            if response.status().is_success() {
                let user: serde_json::Value = response.json().await?;
                println!("{} {} ({})", "✓ Created user".green(), user["username"], user["email"]);
            } else {
                println!("{}: {}", "User creation failed".red(), response.text().await?);
            }
        }
    }

    Ok(())
}