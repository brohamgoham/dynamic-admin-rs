use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::sync::Arc;

use crate::api::DynamicApiClient;
use crate::commands::Command;

// Command to list organizations
pub struct ListOrganizationsCommand {
    api_client: Arc<DynamicApiClient>,
}

impl ListOrganizationsCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for ListOrganizationsCommand {
    fn name(&self) -> &str {
        "list"
    }

    fn description(&self) -> &str {
        "List all organizations"
    }

    async fn execute(&self) -> Result<()> {

        println!("{}", "Fetching organizations...".blue());
        let result = self.api_client.list_organizations().await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}

// Command to get a specific organization
pub struct GetOrganizationCommand {
    api_client: Arc<DynamicApiClient>,
}

impl GetOrganizationCommand {
    pub fn new(api_client: Arc<DynamicApiClient>) -> Self {
        Self { api_client }
    }
}

#[async_trait]
impl Command for GetOrganizationCommand {
    fn name(&self) -> &str {
        "get"
    }

    fn description(&self) -> &str {
        "Get a specific organization"
    }

    async fn execute(&self) -> Result<()> {
        let environment_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter environment ID")
            .interact()?;

        let org_id: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter organization ID")
            .interact()?;

        println!("{}", "Fetching organization...".blue());
        let result = self.api_client.get_organization(&environment_id, &org_id).await?;
        
        // Pretty print the JSON result
        let formatted = serde_json::to_string_pretty(&result)?;
        println!("{}", formatted);
        
        Ok(())
    }
}