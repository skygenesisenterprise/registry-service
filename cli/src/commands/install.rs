use anyhow::Result;
use colored::*;
use dialoguer::Confirm;
use crate::config::Config;

pub async fn execute(name: String, version: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = reqwest::Client::new();
    
    println!("{} {} ({})", "Installing".green().bold(), name.cyan(), version.unwrap_or("latest".to_string()).yellow());
    
    let url = if let Some(v) = version {
        format!("{}/api/packages/{}/{}", config.registry_url, name, v)
    } else {
        format!("{}/api/packages/{}", config.registry_url, name)
    };
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.auth_token.unwrap_or_default()))
        .send()
        .await?;

    if response.status().is_success() {
        let package: serde_json::Value = response.json().await?;
        
        println!("Package found:");
        println!("  Name: {}", package["name"]);
        println!("  Version: {}", package["version"]);
        println!("  Size: {} bytes", package["size"]);
        println!("  Maintainer: {}", package["maintainer"]);
        
        if Confirm::new()
            .with_prompt("Do you want to continue with installation?")
            .default(true)
            .interact()?
        {
            println!("{} {}", "Downloading".blue(), name.cyan());
            
            let download_url = format!("{}/api/packages/{}/download", config.registry_url, package["id"]);
            let download_response = client
                .get(&download_url)
                .header("Authorization", format!("Bearer {}", config.auth_token.unwrap_or_default()))
                .send()
                .await?;
            
            if download_response.status().is_success() {
                let package_data = download_response.bytes().await?;
                let file_path = config.install_dir.join(format!("{}-{}.deb", name, package["version"]));
                
                tokio::fs::write(&file_path, package_data).await?;
                
                println!("{} {} to {}", "Installed".green().bold(), name.cyan(), file_path.display());
            } else {
                println!("{}: {}", "Download failed".red(), download_response.text().await?);
            }
        }
    } else {
        println!("{}: {}", "Package not found".red(), response.text().await?);
    }

    Ok(())
}