use anyhow::Result;
use colored::*;
use crate::config::Config;

pub async fn execute(installed_only: bool) -> Result<()> {
    let config = Config::load();
    
    if installed_only {
        println!("{}", "Installed Packages:".green().bold());
        println!("{}", "─".repeat(50));
        
        let install_dir = &config?.install_dir;
        let mut packages = Vec::new();
        
        for entry in std::fs::read_dir(install_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".deb") {
                    packages.push(filename.to_string());
                }
            }
        }
        
        if packages.is_empty() {
            println!("{}", "No packages installed.".yellow());
        } else {
            for (i, package) in packages.iter().enumerate() {
                println!("{}. {}", i + 1, package.cyan());
            }
        }
    } else {
        let config = config?;
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
                println!("{}", "Available Packages:".green().bold());
                println!("{}", "─".repeat(50));
                
                for (i, package) in packages_array.iter().enumerate() {
                    let name = package["name"].as_str().unwrap_or("Unknown");
                    let version = package["version"].as_str().unwrap_or("Unknown");
                    let maintainer = package["maintainer"].as_str().unwrap_or("Unknown");
                    
                    println!("{}. {} ({})", i + 1, name.cyan(), version.yellow());
                    println!("   Maintainer: {}", maintainer.dimmed());
                    println!();
                }
            }
        } else {
            println!("{}: {}", "Error".red(), response.text().await?);
        }
    }

    Ok(())
}