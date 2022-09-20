use anyhow::Result;
use derive_more::Display;
use serde::Serialize;

use super::{DbClient, MaybeQueryable, Queryable};
use crate::schema::Letter;

#[derive(Display, Serialize)]
#[display(fmt = "CreateLetter({author})")]
pub struct CreateLetterQuery<'a> {
    author: &'a str,
    message: &'a str,
    secret: bool,
}

impl<'a> CreateLetterQuery<'a> {
    pub fn new(author: &'a str, message: &'a str) -> Self {
        Self {
            author,
            message,
            secret: false,
        }
    }

    pub fn secret(self, value: bool) -> Self {
        Self {
            secret: value,
            ..self
        }
    }
}

#[rocket::async_trait]
impl<'a> Queryable for CreateLetterQuery<'a> {
    type Output = Letter;

    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
        Ok(client
            .from("letters")
            .insert(serde_json::to_string(&self)?)
            .single()
            .execute()
            .await?)
    }
}

#[derive(Display, Serialize)]
pub enum GetLetterQuery<'a> {
    #[display(fmt = "GetLetter(id = `{_0}`)")]
    Id(&'a str),

    #[display(fmt = "GetLetter(author = `{_0}`)")]
    Author(&'a str),
}

#[rocket::async_trait]
impl<'a> MaybeQueryable for GetLetterQuery<'a> {
    type Output = Letter;

    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
        let builder = client.from("letters");
        let builder = match self {
            Self::Author(author) => builder.eq("author", author),
            Self::Id(id) => builder.eq("id", id),
        };
        Ok(builder.single().execute().await?)
    }
}
