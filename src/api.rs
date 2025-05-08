use anyhow::Result;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
//use std::sync::Arc;

// API client for DynamicSDK
pub struct DynamicApiClient {
    client: Client,
    api_token: String,
    base_url: String,
}

impl DynamicApiClient {
    pub fn new(api_token: String, base_url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", api_token)).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        DynamicApiClient {
            client,
            api_token,
            base_url,
        }
    }

    // Generic GET request
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API error ({}): {}", status, error_text));
        }
        
        let data = response.json::<T>().await?;
        Ok(data)
    }

    // Generic POST request
    pub async fn post<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.post(&url).json(body).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("API error ({}): {}", status, error_text));
        }
        
        let data = response.json::<T>().await?;
        Ok(data)
    }

    // Organizations
    //   --url https://app.dynamicauth.com/api/v0/organizations \
    pub async fn list_organizations(&self) -> Result<Value> {
        self.get(&format!("/api/v0/environments/organizations")).await
    }

    pub async fn get_organization(&self, environment_id: &str, org_id: &str) -> Result<Value> {
        self.get(&format!("/api/v0/environments/{}/organizations/{}", environment_id, org_id)).await
    }

    // Exports
    pub async fn list_exports(&self, environment_id: &str) -> Result<Value> {
        self.get(&format!("/api/v0/environments/{}/exports", environment_id)).await
    }

    pub async fn get_export(&self, environment_id: &str, export_id: &str) -> Result<Value> {
        self.get(&format!("/api/v0/environments/{}/exports/{}", environment_id, export_id)).await
    }

    pub async fn create_export(&self, environment_id: &str, params: &Value) -> Result<Value> {
        self.post(&format!("/api/v0/environments/{}/exports", environment_id), params).await
    }

    // Users
    pub async fn list_users(&self, environment_id: &str) -> Result<Value> {
        self.get(&format!("/api/v0/environments/{}/users", environment_id)).await
    }

    pub async fn get_user(&self, environment_id: &str, user_id: &str) -> Result<Value> {
        self.get(&format!("/api/v0/environments/{}/users/{}", environment_id, user_id)).await
    }
}