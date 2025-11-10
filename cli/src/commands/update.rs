use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::config::Config;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    
    println!("{}", "Updating package index...".blue().bold());
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap()
    );
    pb.set_message("Fetching package list...");
    
    let client = reqwest::Client::new();
    let url = format!("{}/api/packages", config.registry_url);
    
    pb.inc(1);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.auth_token.unwrap_or_default()))
        .send()
        .await?;

    if response.status().is_success() {
        let packages: serde_json::Value = response.json().await?;
        
        pb.set_message("Caching package information...");
        pb.inc(1);
        
        let cache_file = config.cache_dir.join("packages.json");
        tokio::fs::write(cache_file, serde_json::to_string_pretty(&packages)?).await?;
        
        pb.finish_with_message("Package index updated successfully!");
        println!("{}", "✓ Package index updated".green().bold());
    } else {
        pb.finish_with_message("Failed to update package index");
        println!("{}: {}", "Error".red(), response.text().await?);
    }

    Ok(())
}