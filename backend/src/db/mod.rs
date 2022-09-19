use anyhow::Result;
use rocket::async_trait;
use serde::de::DeserializeOwned;

use crate::util::handle_supabase_error;

pub mod user;
pub use postgrest::Postgrest as DbClient;

/// Creates a new Postgrest API client to make changes
/// or view something to the Postgres database.
pub fn create_db_client(endpoint_url: impl AsRef<str>, api_key: impl AsRef<str>) -> DbClient {
    DbClient::new(format!("{}/rest/v1", endpoint_url.as_ref())).insert_header("apikey", api_key)
}

#[async_trait]
pub trait Queryable: std::fmt::Display
where
    Self: Sized,
{
    type Output: DeserializeOwned;

    /// This is the internal method required to implement if you want to
    /// implement Queryable trait from an object.
    ///
    /// Unlike query method, this is without any wrappers for database error handling.
    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response>;

    /// Performs a query to the database.
    ///
    /// You don't need to implement it manually because it already has internal code
    /// for database error handling and so forth.
    async fn query(self, client: &DbClient) -> Result<Self::Output> {
        let name = self.to_string();
        log::debug!("querying: {}", name);

        let response = self.query_inner(client).await?;
        if response.status().is_success() {
            log::debug!("[{}] query done", name);

            let text = &response.text().await?;
            log::trace!("[{}] received json <= {}", name, text);

            Ok(serde_json::from_str(&text)?)
        } else {
            handle_supabase_error(name, response).await
        }
    }
}
