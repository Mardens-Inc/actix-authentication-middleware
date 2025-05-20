use anyhow::Result;
use serde::{Deserialize, Serialize};

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
        let client = reqwest::Client::new();
        let response = client
            .get("https://lib.mardens.com/auth/users")
            .send()
            .await?;
        let users: Vec<Self> = response.json().await?;
        Ok(users)
    }
    pub async fn query_user_by_name(username: impl AsRef<str>) -> Result<Vec<Self>> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://lib.mardens.com/auth/users?query={}",
                username.as_ref()
            ))
            .send()
            .await?;

        let users: Vec<Self> = response.json().await?;
        Ok(users)
    }
    pub async fn authenticate_user(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Option<String>> {
        let client = reqwest::Client::builder().build()?;

        let form = reqwest::multipart::Form::new()
            .text("username", username.as_ref().to_string())
            .text("password", password.as_ref().to_string());

        let response = client
            .post("https://lib.mardens.com/auth/")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            return Err(anyhow::anyhow!("{}", error_message));
        }

        Ok(json.get("token").and_then(|t| t.as_str()).map(String::from))
    }

    pub async fn authenticate_user_with_token(token: impl AsRef<str>) -> Result<Option<String>> {
        let client = reqwest::Client::builder().build()?;

        let form = reqwest::multipart::Form::new().text("token", token.as_ref().to_string());

        let response = client
            .post("https://lib.mardens.com/auth/")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            return Err(anyhow::anyhow!("{}", error_message));
        }

        Ok(json.get("token").and_then(|t| t.as_str()).map(String::from))
    }

    pub async fn register_user(username: impl AsRef<str>, password: impl AsRef<str>) -> Result<()> {
        let client = reqwest::Client::builder().build()?;

        let form = reqwest::multipart::Form::new()
            .text("username", username.as_ref().to_string())
            .text("password", password.as_ref().to_string());

        let response = client
            .post("https://lib.mardens.com/auth/register")
            .header("Content-Type", "multipart/form-data")
            .header("Accept", "application/json")
            .multipart(form)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        if !json
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
            return Err(anyhow::anyhow!("{}", error_message));
        }

        Ok(())
    }
}
