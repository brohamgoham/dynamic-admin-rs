use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::sync::Arc;

use crate::api::DynamicApiClient;
use crate::commands::Command;

// Command to list users
pub struct ListUsersCommand {
    api_client: Arc<DynamicApiClient>,
}

impl ListUsersCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for ListUsersCommand {
    fn name(&self) -> &str {
        "list"
    }

    fn description(&self) -> &str {
        "List all users"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        println!("{}", "Fetching users...".blue());
        let result = self.api_client.list_users(&environment_id).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}

// Command to get a specific user
pub struct GetUserCommand {
    api_client: Arc<DynamicApiClient>,
}

impl GetUserCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for GetUserCommand {
    fn name(&self) -> &str {
        "get"
    }

    fn description(&self) -> &str {
        "Get a specific user"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        let user_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter user ID")
            .interact()?;

        println!("{}", "Fetching user...".blue());
        let result = self.api_client.get_user(&environment_id, &user_id).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}