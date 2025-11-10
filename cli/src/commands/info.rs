use anyhow::Result;
use colored::*;
use crate::config::Config;

pub async fn execute(name: String, version: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = reqwest::Client::new();
    
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
        
        println!("{}", "Package Information:".green().bold());
        println!("{}", "─".repeat(50));
        println!("{}: {}", "Name".cyan(), package["name"]);
        println!("{}: {}", "Version".cyan(), package["version"]);
        println!("{}: {}", "Description".cyan(), package["description"].as_str().unwrap_or("No description"));
        println!("{}: {}", "Maintainer".cyan(), package["maintainer"]);
        println!("{}: {}", "Architecture".cyan(), package["architecture"]);
        println!("{}: {} bytes", "Size".cyan(), package["size"]);
        println!("{}: {}", "Checksum".cyan(), package["checksum"]);
        println!("{}: {}", "Created".cyan(), package["created_at"]);
        println!("{}: {}", "Updated".cyan(), package["updated_at"]);
        
        if let Some(dependencies) = package["dependencies"].as_array() {
            println!("{}:", "Dependencies".cyan());
            for dep in dependencies {
                println!("  - {} ({})", dep["name"], dep["version"]);
            }
        }
        
        if let Some(tags) = package["tags"].as_array() {
            println!("{}:", "Tags".cyan());
            for tag in tags {
                println!("  - {}", tag["name"]);
            }
        }
    } else {
        println!("{}: {}", "Package not found".red(), response.text().await?);
    }

    Ok(())
}