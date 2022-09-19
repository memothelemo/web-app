use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmissionState {
    pub available: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Letter {
    pub created_at: String,
    pub id: String,
    pub author: String,
    pub message: String,
    pub secret: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub created_at: String,
    pub id: String,
    pub username: String,
    pub password: String,
}
