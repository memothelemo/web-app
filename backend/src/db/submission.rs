use anyhow::Result;
use derive_more::Display;
use rocket::async_trait;
use serde::Serialize;

use super::{DbClient, Queryable};
use crate::schema::SubmissionState;

#[derive(Display, Serialize)]
pub struct SubmissionQuery;

#[async_trait]
impl Queryable for SubmissionQuery {
    type Output = SubmissionState;

    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
        Ok(client.from("state").single().execute().await?)
    }
}
