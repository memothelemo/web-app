use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub salt_code: String,
    pub database_url: String,
    pub database_key: String,
}
