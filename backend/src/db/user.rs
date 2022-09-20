use anyhow::Result;
use derive_more::Display;
use rocket::async_trait;
use serde::Serialize;

use super::{DbClient, MaybeQueryable, Queryable};
use crate::schema::User;

#[derive(Display, Serialize)]
pub enum GetUserQuery<'a> {
    #[display(fmt = "GetUsername::Username({_0})")]
    Username(&'a str),

    #[display(fmt = "GetUsername::Id({_0})")]
    Id(&'a str),
}

#[async_trait]
impl<'a> MaybeQueryable for GetUserQuery<'a> {
    type Output = User;

    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
        let builder = client.from("users");
        let builder = match self {
            Self::Username(name) => builder.eq("username", name),
            Self::Id(id) => builder.eq("id", id),
        };
        Ok(builder.single().execute().await?)
    }
}

#[derive(Display, Serialize)]
#[display(fmt = "RegisterUser({username})")]
pub struct RegisterUserQuery<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> RegisterUserQuery<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Self { username, password }
    }
}

#[async_trait]
impl<'a> Queryable for RegisterUserQuery<'a> {
    type Output = User;

    async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
        Ok(client
            .from("users")
            .insert(serde_json::to_string(&self).unwrap())
            .single()
            .execute()
            .await?)
    }
}
