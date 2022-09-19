use anyhow::{anyhow, Result};
use serde::Deserialize;

use super::get_content_type;

/// Structure object for a Postgrest API error.
#[derive(Debug, Deserialize)]
pub struct SupabaseError {
    pub hint: Option<String>,
    pub code: String,
    pub details: Option<String>,
    pub message: String,
}

/// Handles any Supabase postgrest API errors.
pub async fn handle_supabase_error<T>(
    query_name: impl AsRef<str>,
    response: reqwest::Response,
) -> Result<T> {
    let query_name = query_name.as_ref();
    let (message, code) = if get_content_type(response.headers())
        .map(|v| v.subtype() == mime::JSON)
        .unwrap_or_default()
    {
        let error = serde_json::from_str::<SupabaseError>(&response.text().await?)?;
        log::debug!(
            "{} => query error ({}): {}",
            query_name,
            error.code,
            error.message
        );

        (error.message, error.code)
    } else {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("<unknown error>"));
        log::debug!("{} => query error (none)", query_name);
        (message, String::from("<none>"))
    };
    Err(anyhow!("{}: {}", code, message))
}
