use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_json::json;
use std::sync::Arc;

use crate::api::DynamicApiClient;
use crate::commands::Command;

// Command to list exports
pub struct ListExportsCommand {
    api_client: Arc<DynamicApiClient>,
}

impl ListExportsCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for ListExportsCommand {
    fn name(&self) -> &str {
        "list"
    }

    fn description(&self) -> &str {
        "List all exports"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        println!("{}", "Fetching exports...".blue());
        let result = self.api_client.list_exports(&environment_id).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}

// Command to get a specific export
pub struct GetExportCommand {
    api_client: Arc<DynamicApiClient>,
}

impl GetExportCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for GetExportCommand {
    fn name(&self) -> &str {
        "get"
    }

    fn description(&self) -> &str {
        "Get a specific export"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        let export_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter export ID")
            .interact()?;

        println!("{}", "Fetching export...".blue());
        let result = self.api_client.get_export(&environment_id, &export_id).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}

// Command to create a new export
pub struct CreateExportCommand {
    api_client: Arc<DynamicApiClient>,
}

impl CreateExportCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for CreateExportCommand {
    fn name(&self) -> &str {
        "create"
    }

    fn description(&self) -> &str {
        "Create a new export"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        // Select export type
        let export_types = vec!["users", "organizations", "wallets"];
        let selected_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select export type")
            .default(0)
            .items(&export_types)
            .interact()?;

        // Create export request body
        let export_params = json!({
            "type": export_types[selected_type],
            "format": "csv"
        });

        println!("{}", "Creating export...".blue());
        let result = self.api_client.create_export(&environment_id, &export_params).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}