use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::env;
use std::process;

mod commands;
mod config;
mod api;
mod command;

use commands::CommandRegistry;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "⭐️ DynamicSDK Admin Operations ⭐️".bold().cyan());
    println!("{}", r"
         __                                 _                           
    ____/ /__  __ ____   ____ _ ____ ___   (_)_____    _  __ __  __ ____
   / __  // / / // __ \ / __ `// __ `__ \ / // ___/   | |/_// / / //_  /
  / /_/ // /_/ // / / // /_/ // / / / / // // /__ _  _>  < / /_/ /  / /_
  \__,_/ \__, //_/ /_/ \__,_//_/ /_/ /_//_/ \___/(_)/_/|_| \__, /  /___/
        /____/                                            /____/                                                                                                             
    ".magenta());

    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}: {}", "Error loading configuration".red(), e);
            process::exit(1);
        }
    };

    let registry = commands::create_command_registry(config);
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd_path = &args[1..];
        match execute_command_from_args(&registry, cmd_path).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}: {}", "Error".red(), e);
                process::exit(1);
            }
        }
        return Ok(());
    }

    // Interactive mode - select category first
    let mut current_registry = &registry;
    let mut path = Vec::new();

    loop {
        // If we're at leaf level with an executable command
        if current_registry.categories.is_empty() && current_registry.commands.len() == 1 {
            let cmd = &current_registry.commands[0];
            println!("\n{} {}", "Executing:".cyan(), path.join(" ").yellow());
            match cmd.execute().await {
                Ok(_) => println!("{}", "Command executed successfully".green()),
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
            break;
        }

        // Display available categories and commands
        let mut options = Vec::new();
        
        // Add back option if we're in a subcategory
        if !path.is_empty() {
            options.push("[Back]");
        }
        
        // Add exit option
        options.push("[Exit]");
        
        // Add categories
        for category in &current_registry.categories {
            options.push(&category.name);
        }
        
        // Add commands
        for cmd in &current_registry.commands {
            options.push(&cmd.name());
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&options)
            .interact()?;

        // Handle back option
        if !path.is_empty() && selection == 0 {
            path.pop();
            current_registry = get_registry_at_path(&registry, &path);
            continue;
        }

        // Handle exit option
        let exit_idx = if path.is_empty() { 0 } else { 1 };
        if selection == exit_idx {
            println!("{}", "Goodbye!".cyan());
            break;
        }

        // Adjust index for back/exit options
        let adjusted_idx = if path.is_empty() { selection - 1 } else { selection - 2 };

        // Check if selection is a category
        if adjusted_idx < current_registry.categories.len() {
            let category = &current_registry.categories[adjusted_idx];
            path.push(category.name.clone());
            current_registry = &category.registry;
        } else {
            // Selection is a command
            let cmd_idx = adjusted_idx - current_registry.categories.len();
            let cmd = &current_registry.commands[cmd_idx];
            println!("\n{} {}", "Executing:".cyan(), cmd.name().yellow());
            match cmd.execute().await {
                Ok(_) => println!("{}", "Command executed successfully".green()),
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
            
            // For interactive flow, we continue the loop after executing a command
        }
    }

    Ok(())
}

// Get registry at a specific path
fn get_registry_at_path<'a>(registry: &'a CommandRegistry, path: &[String]) -> &'a CommandRegistry {
    let mut current = registry;
    for segment in path {
        let found = current.categories.iter().find(|c| &c.name == segment);
        if let Some(category) = found {
            current = &category.registry;
        } else {
            return current; // Path not found, return current registry
        }
    }
    current
}

// Execute command from command-line arguments
async fn execute_command_from_args(registry: &CommandRegistry, cmd_path: &[String]) -> Result<()> {
    let mut current = registry;
    let path_len = cmd_path.len();
    
    // Navigate to the appropriate registry
    for (i, segment) in cmd_path.iter().enumerate() {
        // If this is the last segment, it might be a command
        if i == path_len - 1 {
            // Try to find and execute command
            if let Some(cmd) = current.commands.iter().find(|c| &c.name() == segment) {
                return cmd.execute().await;
            }
        }
        
        // Otherwise, it should be a category
        if let Some(category) = current.categories.iter().find(|c| &c.name == segment) {
            current = &category.registry;
        } else {
            return Err(anyhow::anyhow!("Invalid command path: {}", cmd_path.join(" ")));
        }
    }
    
    Err(anyhow::anyhow!("No command found at path: {}", cmd_path.join(" ")))
}