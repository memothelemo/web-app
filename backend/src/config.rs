use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub access_token: String,
    pub database_url: String,
    pub database_key: String,
}
