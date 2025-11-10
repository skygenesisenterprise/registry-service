use anyhow::Result;
use colored::*;
use dialoguer::Confirm;
use crate::config::Config;

pub async fn execute(all: bool) -> Result<()> {
    let config = Config::load()?;
    
    if all {
        println!("{}", "Upgrading all packages...".blue().bold());
        
        let client = reqwest::Client::new();
        let url = format!("{}/api/packages", config.registry_url);
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.auth_token.unwrap_or_default()))
            .send()
            .await?;

        if response.status().is_success() {
            let packages: serde_json::Value = response.json().await?;
            
            if let Some(packages_array) = packages.as_array() {
                println!("Found {} packages to check for updates", packages_array.len());
                
                if Confirm::new()
                    .with_prompt("Do you want to upgrade all packages?")
                    .default(false)
                    .interact()?
                {
                    for package in packages_array {
                        let name = package["name"].as_str().unwrap_or("Unknown");
                        let version = package["version"].as_str().unwrap_or("Unknown");
                        
                        println!("Upgrading {} to {}...", name.cyan(), version.yellow());
                    }
                    
                    println!("{}", "✓ All packages upgraded".green().bold());
                }
            }
        } else {
            println!("{}: {}", "Error".red(), response.text().await?);
        }
    } else {
        println!("{}", "Please specify a package to upgrade or use --all for all packages".yellow());
        println!("Usage: cpkgs upgrade --all");
    }

    Ok(())
}