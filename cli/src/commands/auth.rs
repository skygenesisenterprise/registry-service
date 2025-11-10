use anyhow::Result;
use colored::*;
use dialoguer::{Input, Password};
use serde_json::json;
use crate::config::Config;
use crate::main::AuthAction;

pub async fn execute(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => {
            let username: String = Input::new()
                .with_prompt("Username")
                .interact()?;
            
            let password: String = Password::new()
                .with_prompt("Password")
                .interact()?;
            
            let config = Config::load()?;
            let client = reqwest::Client::new();
            
            let url = format!("{}/api/auth/login", config.registry_url);
            
            let response = client
                .post(&url)
                .json(&json!({
                    "username": username,
                    "password": password
                }))
                .send()
                .await?;

            if response.status().is_success() {
                let auth_response: serde_json::Value = response.json().await?;
                let token = auth_response["token"].as_str().unwrap_or("");
                
                let mut config = config;
                config.set_auth_token(token.to_string());
                config.save()?;
                
                println!("{} {}!", "Welcome back".green().bold(), username.cyan());
                println!("{}", "✓ Successfully logged in".green());
            } else {
                println!("{}: {}", "Login failed".red(), response.text().await?);
            }
        }
        
        AuthAction::Logout => {
            let mut config = Config::load()?;
            config.clear_auth_token();
            config.save()?;
            
            println!("{}", "✓ Successfully logged out".green());
        }
        
        AuthAction::Register => {
            let username: String = Input::new()
                .with_prompt("Username")
                .interact()?;
            
            let email: String = Input::new()
                .with_prompt("Email")
                .interact()?;
            
            let password: String = Password::new()
                .with_prompt("Password")
                .with_confirmation("Confirm password", "Passwords don't match")
                .interact()?;
            
            let config = Config::load()?;
            let client = reqwest::Client::new();
            
            let url = format!("{}/api/auth/register", config.registry_url);
            
            let response = client
                .post(&url)
                .json(&json!({
                    "username": username,
                    "email": email,
                    "password": password
                }))
                .send()
                .await?;

            if response.status().is_success() {
                let auth_response: serde_json::Value = response.json().await?;
                let token = auth_response["token"].as_str().unwrap_or("");
                
                let mut config = config;
                config.set_auth_token(token.to_string());
                config.save()?;
                
                println!("{} {}!", "Welcome".green().bold(), username.cyan());
                println!("{}", "✓ Successfully registered and logged in".green());
            } else {
                println!("{}: {}", "Registration failed".red(), response.text().await?);
            }
        }
        
        AuthAction::Status => {
            let config = Config::load()?;
            
            if let Some(_token) = config.auth_token {
                println!("{}", "✓ You are logged in".green());
                println!("Registry: {}", config.registry_url.cyan());
            } else {
                println!("{}", "✗ You are not logged in".red());
                println!("Use 'cpkgs auth login' to authenticate");
            }
        }
    }

    Ok(())
}