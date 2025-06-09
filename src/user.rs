use anyhow::Result;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
        info!(
            "Found {} users matching query '{}'",
            users.len(),
            username.as_ref()
        );
        Ok(users)
    }

    pub async fn authenticate_user(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        user_agent: impl AsRef<str>,
    ) -> Result<Option<String>> {
        debug!("Attempting to authenticate user: {}", username.as_ref());
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let form = [
            ("username", username.as_ref()),
            ("password", password.as_ref()),
        ];

        trace!(
            "Sending authentication request for user {}",
            username.as_ref()
        );
        let response = client
            .post("https://lib.mardens.com/auth/")
            .form(&form) // Use form instead of multipart
            .header("Accept", "application/json")
            .header("User-Agent", user_agent.as_ref())
            .send()
            .await?;

        trace!(
            "Received authentication response with status: {}",
            response.status()
        );

        // Parse the JSON response
        let json: serde_json::Value = response.json().await?;

        // Check the success flag
        let success = json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false);

        // Get message and token values
        let message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
        let token = json.get("token").and_then(|t| t.as_str()).map(String::from);

        if !success {
            // Log the error but don't return an error
            error!(
                "Authentication failed for user {}: {}",
                username.as_ref(),
                message
            );
            return Ok(None);
        }

        if token.is_some() {
            info!("User {} successfully authenticated", username.as_ref());
        } else {
            warn!(
                "Authentication succeeded but no token returned for user {}",
                username.as_ref()
            );
        }

        Ok(token)
    }

    pub async fn authenticate_user_with_token(
        token: impl AsRef<str>,
        user_agent: impl AsRef<str>,
    ) -> Result<()> {
        debug!("Authenticating with token: {}", token.as_ref());
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))?;

        trace!("Sending token authentication request to https://lib.mardens.com/auth/");
        let response = client
            .post("https://lib.mardens.com/auth/")
            .header("Accept", "application/json")
            .header("User-Agent", user_agent.as_ref())
            .form(&[("token", token.as_ref())])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send authentication request: {}", e))?;
        let status = response.status();

        trace!(
            "Received token authentication response with status: {} ({})",
            response.status().as_u16(),
            response.status()
        );
        let body = response.text().await.map_err(|e| anyhow::anyhow!("Failed to read response body: {}", e))?;
        let json = serde_json::from_str::<serde_json::Value>(&body).map_err(|e| anyhow::anyhow!(json!({
            "message": "Failed to parse JSON response",
            "error": e.to_string(),
            "body": body,
        })))?;

        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown authentication error");
            error!("Token authentication failed - Status: {}, Message: {}", status, error_message);
            if status.is_client_error() {
                return Err(anyhow::anyhow!(json!({
                    "error": "Client error",
                    "message": error_message,
                    "status": status.as_u16(),
                    "body": body,
                })));
            }
            return Err(anyhow::anyhow!(json!({
                "error": "Authentication failed",
                "message": error_message,
                "status": status.as_u16(),
                "body": body,
            })));
        }

        debug!("Token authentication successful");
        Ok(())
    }

    pub async fn register_user(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        user_agent: impl AsRef<str>,
    ) -> Result<()> {
        info!("Registering new user: {}", username.as_ref());
        let client = reqwest::Client::builder().build()?;

        let form = [
            ("username", username.as_ref()),
            ("password", password.as_ref()),
        ];

        debug!(
            "Sending registration request for user {}",
            username.as_ref()
        );
        let response = client
            .post("https://lib.mardens.com/auth/register")
            .form(&form) // Use form instead of multipart
            .header("Accept", "application/json")
            .header("User-Agent", user_agent.as_ref())
            .send()
            .await?;

        trace!(
            "Received registration response with status: {}",
            response.status()
        );

        // Parse the JSON response
        let json: serde_json::Value = response.json().await?;

        // Check the success flag
        let success = json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false);

        if !success {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            error!(
                "User registration failed for {}: {}",
                username.as_ref(),
                error_message
            );
            return Err(anyhow::anyhow!("{}", error_message));
        }

        info!("Successfully registered user {}", username.as_ref());
        Ok(())
    }
}
