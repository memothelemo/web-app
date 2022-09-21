use anyhow::{anyhow, Result};
use rocket::async_trait;
use serde::de::DeserializeOwned;

use crate::util::postgrest_err_info;

#[cfg(test)]
mod tests;

pub mod letters;
pub mod submission;
pub mod user;

pub use postgrest::Postgrest as DbClient;

/// Creates a new Postgrest API client to make changes
/// or view something to the Postgres database.
pub fn create_db_client(endpoint_url: impl AsRef<str>, api_key: impl AsRef<str>) -> DbClient {
    DbClient::new(format!("{}/rest/v1", endpoint_url.as_ref())).insert_header("apikey", api_key)
}

#[async_trait]
pub trait MaybeQueryable: std::fmt::Display
where
    Self: Sized,
{
    type Output: DeserializeOwned;

    /// This is the internal method required to implement if you want to
    /// implement MaybeQueryable trait from an object.e
    ///
    /// Unlike query method, this is without any wrappers for database error handling.
    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response>;

    /// Performs a query to the database.
    ///
    /// It will return an Option because query cannot be made
    /// because it doesn't exists.
    ///
    /// You don't need to implement it manually because it already has internal code
    /// for database error handling and so forth.
    async fn query(self, client: &DbClient) -> Result<Option<Self::Output>> {
        let name = self.to_string();
        log::info!("querying (maybe): {}", name);

        let response = self.query_inner(client).await?;
        if response.status().is_success() {
            log::info!("[{}] query done", name);

            let text = &response.text().await?;
            log::trace!("[{}] received json <= {}", name, text);

            Ok(Some(serde_json::from_str(text)?))
        } else {
            let (message, code) = postgrest_err_info(&name, response).await?;
            if code == "PGRST116" {
                Ok(None)
            } else {
                log::error!("[{}] [{}] {}", name, code, message);
                Err(anyhow!("{}: {}", message, code))
            }
        }
    }
}

#[async_trait]
pub trait Queryable: std::fmt::Display
where
    Self: Sized,
{
    type Output: DeserializeOwned;

    /// This is the internal method required to implement if you want to
    /// implement Queryable trait from an object.e
    ///
    /// Unlike query method, this is without any wrappers for database error handling.
    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response>;

    /// Performs a query to the database.
    ///
    /// You don't need to implement it manually because it already has internal code
    /// for database error handling and so forth.
    async fn query(self, client: &DbClient) -> Result<Self::Output> {
        let name = self.to_string();
        log::info!("querying: {}", name);

        let response = self.query_inner(client).await?;
        if response.status().is_success() {
            log::info!("[{}] query done", name);

            let text = &response.text().await?;
            log::trace!("[{}] received json <= {}", name, text);

            Ok(serde_json::from_str(text)?)
        } else {
            let (message, code) = postgrest_err_info(&name, response).await?;
            log::error!("[{}] [{}] {}", name, code, message);
            Err(anyhow!("{}: {}", message, code))
        }
    }
}
