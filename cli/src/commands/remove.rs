use anyhow::Result;
use colored::*;
use dialoguer::Confirm;
use crate::config::Config;

pub async fn execute(name: String, version: Option<String>) -> Result<()> {
    let config = Config::load()?;
    
    println!("{} {} ({})", "Removing".red().bold(), name.cyan(), version.unwrap_or("all versions".to_string()).yellow());
    
    let package_pattern = if let Some(v) = version {
        format!("{}-{}.deb", name, v)
    } else {
        format!("{}-*.deb", name)
    };
    
    let install_dir = &config.install_dir;
    let mut found_packages = Vec::new();
    
    for entry in std::fs::read_dir(install_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with(&format!("{}-", name)) && filename.ends_with(".deb") {
                if version.is_none() || filename.contains(&format!("-{}.deb", version.as_ref().unwrap())) {
                    found_packages.push(path);
                }
            }
        }
    }
    
    if found_packages.is_empty() {
        println!("{} {}", "No installed packages found matching".yellow(), name.cyan());
        return Ok(());
    }
    
    println!("Found packages to remove:");
    for package in &found_packages {
        println!("  {}", package.display());
    }
    
    if Confirm::new()
        .with_prompt("Do you want to remove these packages?")
        .default(false)
        .interact()?
    {
        for package in found_packages {
            std::fs::remove_file(&package)?;
            println!("{} {}", "Removed".red(), package.display());
        }
    }

    Ok(())
}