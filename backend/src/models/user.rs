use crate::models::prelude::*;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub password: String,
    pub moderator: Option<bool>,
    pub viewer: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
}
