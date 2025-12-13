//! Piplop CLI
//!
//! Command-line tool for working with Piplop storyboards and IP registration.

use clap::{Parser, Subcommand};
use piplop_sdk::{PiplopClient, Storyboard};

#[derive(Parser)]
#[command(name = "piplop")]
#[command(about = "Piplop SDK CLI - Register video content as IP on Story Protocol")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// API base URL
    #[arg(long, env = "PIPLOP_API_URL", default_value = "http://localhost:8080")]
    api_url: String,
    
    /// API key for authentication
    #[arg(long, env = "PIPLOP_API_KEY")]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a storyboard JSON file
    Validate {
        /// Path to storyboard JSON file
        #[arg(short, long)]
        storyboard: String,
    },
    /// Register a storyboard as IP on Story Protocol
    Register {
        /// Path to storyboard JSON file
        #[arg(short, long)]
        storyboard: String,
    },
    /// Check IP registration status
    Status {
        /// Story Protocol IP ID
        #[arg(long)]
        ip_id: String,
    },
    /// Generate JSON schema for storyboard format
    Schema,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Validate { storyboard } => {
            match Storyboard::from_file(&storyboard) {
                Ok(sb) => {
                    println!("✓ Valid storyboard: {}", sb.title);
                    println!("  Duration: {:.1}s", sb.duration);
                    println!("  Layers: {}", sb.layers.len());
                }
                Err(e) => {
                    eprintln!("✗ Invalid storyboard: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Register { storyboard } => {
            let sb = Storyboard::from_file(&storyboard)?;
            let mut client = PiplopClient::new(&cli.api_url);
            if let Some(key) = cli.api_key {
                client = client.with_api_key(&key);
            }
            
            println!("Registering storyboard: {}", sb.title);
            match client.register_storyboard(&sb).await {
                Ok(ip_id) => {
                    println!("✓ Registered as IP: {}", ip_id);
                }
                Err(e) => {
                    eprintln!("✗ Registration failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Status { ip_id } => {
            let client = PiplopClient::new(&cli.api_url);
            match client.get_ip_status(&ip_id).await {
                Ok(status) => {
                    println!("{}", serde_json::to_string_pretty(&status)?);
                }
                Err(e) => {
                    eprintln!("✗ Failed to get status: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Schema => {
            let schema = Storyboard::json_schema();
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
    }
    
    Ok(())
}
