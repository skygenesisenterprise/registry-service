use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "cpkgs")]
#[command(about = "Package Registry CLI for Sky Genesis Enterprise")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for packages
    Search {
        query: String,
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Install a package
    Install {
        name: String,
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Remove a package
    Remove {
        name: String,
        #[arg(short, long)]
        version: Option<String>,
    },
    /// List installed packages
    List {
        #[arg(short, long)]
        installed: bool,
    },
    /// Show package information
    Info {
        name: String,
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Update package index
    Update,
    /// Upgrade packages
    Upgrade {
        #[arg(short, long)]
        all: bool,
    },
    /// User authentication
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Package management (admin only)
    Admin {
        #[command(subcommand)]
        action: AdminAction,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    Login,
    Logout,
    Register,
    Status,
}

#[derive(Subcommand)]
enum AdminAction {
    Upload {
        package_file: String,
    },
    Remove {
        name: String,
        version: String,
    },
    ListUsers,
    CreateUser {
        username: String,
        email: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query, limit } => {
            commands::search::execute(query, limit).await?;
        }
        Commands::Install { name, version } => {
            commands::install::execute(name, version).await?;
        }
        Commands::Remove { name, version } => {
            commands::remove::execute(name, version).await?;
        }
        Commands::List { installed } => {
            commands::list::execute(installed).await?;
        }
        Commands::Info { name, version } => {
            commands::info::execute(name, version).await?;
        }
        Commands::Update => {
            commands::update::execute().await?;
        }
        Commands::Upgrade { all } => {
            commands::upgrade::execute(all).await?;
        }
        Commands::Auth { action } => {
            commands::auth::execute(action).await?;
        }
        Commands::Admin { action } => {
            commands::admin::execute(action).await?;
        }
    }

    Ok(())
}