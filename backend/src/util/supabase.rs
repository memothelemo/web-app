use anyhow::Result;
use serde::Deserialize;

/// Structure object for a Postgrest API error.
#[derive(Debug, Deserialize)]
pub struct SupabaseError {
    pub hint: Option<String>,
    pub code: String,
    pub details: Option<String>,
    pub message: String,
}

/// Handles any Supabase postgrest API errors.
pub async fn postgrest_err_info(
    query_name: impl AsRef<str>,
    response: reqwest::Response,
) -> Result<(String, String)> {
    let query_name = query_name.as_ref();
    if response
        .headers()
        .get("content-type")
        .map(|v| {
            if let Ok(str) = v.to_str() {
                str.ends_with("json; charset=utf-8")
            } else {
                false
            }
        })
        .unwrap_or_default()
    {
        let error = serde_json::from_str::<SupabaseError>(&response.text().await?)?;
        log::debug!(
            "{} => query error ({}): {}",
            query_name,
            error.code,
            error.message
        );

        Ok((error.message, error.code))
    } else {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("<unknown error>"));
        log::debug!("{} => query error (none)", query_name);
        Ok((message, String::from("<none>")))
    }
}
