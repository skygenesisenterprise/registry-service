use anyhow::Result;
use colored::*;
use serde_json::Value;
use crate::config::Config;

pub async fn execute(query: String, limit: Option<usize>) -> Result<()> {
    let config = Config::load()?;
    let client = reqwest::Client::new();
    
    let url = format!("{}/api/packages/search/{}", config.registry_url, query);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.auth_token.unwrap_or_default()))
        .send()
        .await?;

    if response.status().is_success() {
        let packages: Vec<Value> = response.json().await?;
        
        if packages.is_empty() {
            println!("{}", "No packages found.".red());
        } else {
            println!("{}", "Search Results:".green().bold());
            println!("{}", "─".repeat(50));
            
            for (i, package) in packages.iter().take(limit.unwrap_or(10)).enumerate() {
                let name = package["name"].as_str().unwrap_or("Unknown");
                let version = package["version"].as_str().unwrap_or("Unknown");
                let description = package["description"].as_str().unwrap_or("No description");
                
                println!("{}. {} ({})", i + 1, name.cyan(), version.yellow());
                println!("   {}", description.dimmed());
                println!();
            }
        }
    } else {
        println!("{}: {}", "Error".red(), response.text().await?);
    }

    Ok(())
}