use anyhow::Result;
use serde::{Deserialize, Serialize};
use log::{debug, error, info, trace, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub reg_date: String,
    pub last_online: String,
    pub last_ip: String,
    pub last_user_agent: String,
    #[serde(rename = "admin")]
    pub is_admin: bool,
}

impl User {
    pub async fn get_users() -> Result<Vec<Self>> {
        debug!("Fetching all users");
        let client = reqwest::Client::new();
        let response = client
            .get("https://lib.mardens.com/auth/users")
            .send()
            .await?;
        
        trace!("Received response with status: {}", response.status());
        
        if !response.status().is_success() {
            warn!("Failed to get users: HTTP {}", response.status());
        }
        
        let users: Vec<Self> = response.json().await?;
        info!("Successfully retrieved {} users", users.len());
        Ok(users)
    }

    pub async fn query_user_by_name(username: impl AsRef<str>) -> Result<Vec<Self>> {
        debug!("Querying users by name: {}", username.as_ref());
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://lib.mardens.com/auth/users?query={}",
                username.as_ref()
            ))
            .send()
            .await?;
        
        trace!("Received response with status: {}", response.status());
        
        if !response.status().is_success() {
            warn!("Failed to query users: HTTP {}", response.status());
        }
        
        let users: Vec<Self> = response.json().await?;
        info!("Found {} users matching query '{}'", users.len(), username.as_ref());
        Ok(users)
    }

    pub async fn authenticate_user(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Option<String>> {
        debug!("Attempting to authenticate user: {}", username.as_ref());
        let client = reqwest::Client::builder().build()?;
        let form = reqwest::multipart::Form::new()
            .text("username", username.as_ref().to_string())
            .text("password", password.as_ref().to_string());
        
        trace!("Sending authentication request for user {}", username.as_ref());
        let response = client
            .post("https://lib.mardens.com/auth/")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;
        
        trace!("Received authentication response with status: {}", response.status());
        
        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            error!("Authentication failed for user {}: {}", username.as_ref(), error_message);
            return Err(anyhow::anyhow!("{}", error_message));
        }
        
        let token = json.get("token").and_then(|t| t.as_str()).map(String::from);
        if token.is_some() {
            info!("User {} successfully authenticated", username.as_ref());
        } else {
            warn!("Authentication succeeded but no token returned for user {}", username.as_ref());
        }
        
        Ok(token)
    }

    pub async fn authenticate_user_with_token(token: impl AsRef<str>) -> Result<Option<String>> {
        debug!("Authenticating with token");
        let client = reqwest::Client::builder().build()?;
        let form = reqwest::multipart::Form::new().text("token", token.as_ref().to_string());
        
        trace!("Sending token authentication request");
        let response = client
            .post("https://lib.mardens.com/auth/")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;
        
        trace!("Received token authentication response with status: {}", response.status());
        
        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            error!("Token authentication failed: {}", error_message);
            return Err(anyhow::anyhow!("{}", error_message));
        }
        
        let new_token = json.get("token").and_then(|t| t.as_str()).map(String::from);
        if new_token.is_some() {
            info!("Successfully authenticated with token");
        } else {
            warn!("Token authentication succeeded but no new token returned");
        }
        
        Ok(new_token)
    }

    pub async fn register_user(username: impl AsRef<str>, password: impl AsRef<str>) -> Result<()> {
        info!("Registering new user: {}", username.as_ref());
        let client = reqwest::Client::builder().build()?;
        let form = reqwest::multipart::Form::new()
            .text("username", username.as_ref().to_string())
            .text("password", password.as_ref().to_string());
        
        debug!("Sending registration request for user {}", username.as_ref());
        let response = client
            .post("https://lib.mardens.com/auth/register")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;
        
        trace!("Received registration response with status: {}", response.status());
        
        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            error!("User registration failed for {}: {}", username.as_ref(), error_message);
            return Err(anyhow::anyhow!("{}", error_message));
        }
        
        info!("Successfully registered user {}", username.as_ref());
        Ok(())
    }
}