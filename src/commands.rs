use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::config::Config;
use crate::api;

// Command trait - the core of our Command pattern
#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self) -> Result<()>;
}

// Command Registry - stores commands and categories
pub struct CommandRegistry {
    pub commands: Vec<Box<dyn Command>>,
    pub categories: Vec<CommandCategory>,
}

// Category of commands
pub struct CommandCategory {
    pub name: String,
    pub description: String,
    pub registry: CommandRegistry,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            commands: Vec::new(),
            categories: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn add_category(&mut self, category: CommandCategory) {
        self.categories.push(category);
    }
}

// Create and populate command registry with all commands
pub fn create_command_registry(config: Config) -> CommandRegistry {
    let api_client = Arc::new(api::DynamicApiClient::new(config.api_token, config.base_url));
    
    let mut registry = CommandRegistry::new();
    
    // Organizations category
    let mut org_registry = CommandRegistry::new();
    org_registry.add_command(Box::new(crate::command::organizations::ListOrganizationsCommand::new(api_client.clone())));
    org_registry.add_command(Box::new(crate::command::organizations::GetOrganizationCommand::new(api_client.clone())));
    
    registry.add_category(CommandCategory {
        name: "organizations".to_string(),
        description: "Organization related commands".to_string(),
        registry: org_registry,
    });
    
    // Exports category
    let mut exports_registry = CommandRegistry::new();
    exports_registry.add_command(Box::new(crate::command::exports::ListExportsCommand::new(api_client.clone())));
    exports_registry.add_command(Box::new(crate::command::exports::GetExportCommand::new(api_client.clone())));
    exports_registry.add_command(Box::new(crate::command::exports::CreateExportCommand::new(api_client.clone())));
    
    registry.add_category(CommandCategory {
        name: "exports".to_string(),
        description: "Export related commands".to_string(),
        registry: exports_registry,
    });
    
    // Users category
    let mut users_registry = CommandRegistry::new();
    users_registry.add_command(Box::new(crate::command::users::ListUsersCommand::new(api_client.clone())));
    users_registry.add_command(Box::new(crate::command::users::GetUserCommand::new(api_client.clone())));
    
    registry.add_category(CommandCategory {
        name: "users".to_string(),
        description: "User related commands".to_string(),
        registry: users_registry,
    });

    registry
}

