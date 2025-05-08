use anyhow::Result;
use config::{Config as ConfigFile, File};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::Deserialize;
use std::path::PathBuf;
use std::io::Write;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_token: String,
    pub base_url: String,
    pub default_environment_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_token: String::new(),
            base_url: "https://app.dynamicauth.com".to_string(),
            default_environment_id: String::new(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    
    // If config doesn't exist, create it
    if !config_path.exists() {
        create_initial_config(&config_path)?;
    }
    
    // Load config
    let config = ConfigFile::builder()
        .add_source(File::from(config_path))
        .build()?;
        
    let config: Config = config.try_deserialize()?;
    
    // Validate config
    if config.api_token.is_empty() {
        return Err(anyhow::anyhow!("API token is missing in configuration"));
    }
    
    if config.default_environment_id.is_empty() {
        return Err(anyhow::anyhow!("Default environment ID is missing in configuration"));
    }
    
    Ok(config)
}

fn get_config_path() -> Result<PathBuf> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    
    path.push("dynamic-admin-ops");
    
    // Create directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    
    path.push("config.toml");
    Ok(path)
}

fn create_initial_config(config_path: &PathBuf) -> Result<()> {
    println!("No configuration found. Let's set up your DynamicSDK Admin CLI.");
    
    let api_token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your API token (starts with dyn_)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("dyn_") && input.len() > 10 {
                Ok(())
            } else {
                Err("Invalid API token format. It should start with 'dyn_'")
            }
        })
        .interact()?;
    
    let base_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the base URL")
        .default("https://app.dynamicauth.com".to_string())
        .interact()?;
    
    let default_environment_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your default environment ID (UUID format)")
        .validate_with(|input: &String| -> Result<(), &str> {
            // Simple UUID format validation
            if input.len() == 36 && input.chars().filter(|&c| c == '-').count() == 4 {
                Ok(())
            } else {
                Err("Invalid UUID format")
            }
        })
        .interact()?;
    
    let config_content = format!(
        "api_token = \"{}\"\n\
         base_url = \"{}\"\n\
         default_environment_id = \"{}\"\n",
        api_token, base_url, default_environment_id
    );
    
    // Create config file and write content
    let mut file = fs::File::create(config_path)?;
    file.write_all(config_content.as_bytes())?;
    
    println!("Configuration saved to {:?}", config_path);
    
    Ok(())
}