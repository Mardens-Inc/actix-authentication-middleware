use crate::User;
use anyhow::{anyhow, Result};
use base64::prelude::*;
use database_common_lib::database_connection::DatabaseConnectionData;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::ConnectOptions;

impl User {
    pub async fn get_user_from_token(token: impl Into<String>) -> Result<Option<User>> {
        let value = BASE64_STANDARD.decode(&token.into())?;
        let value = String::from_utf8(value)?;
        let value: serde_json::Value = serde_json::from_str(&value)?;
        let username: String = value
            .get("username")
            .ok_or(anyhow!("username not found"))?
            .as_str()
            .ok_or(anyhow!("username not found"))?
            .to_string();

        let conn_data = DatabaseConnectionData::get().await?;
        let options = MySqlConnectOptions::new()
            .log_statements(log::LevelFilter::Trace)
            .host(&conn_data.host)
            .username(&conn_data.user)
            .password(&conn_data.password)
            .database("auth");
        let pool = MySqlPoolOptions::new().connect_with(options).await?;
        let user: Option<User> =
            sqlx::query_as(r#"select * from users where username = ? limit 1"#)
                .bind(&username)
                .fetch_optional(&pool)
                .await?;

        Ok(user)
    }
}
