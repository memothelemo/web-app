use crate::models::prelude::*;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
pub struct Letter {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub author: String,
    pub message: String,
    pub secret: bool,
}

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = letters)]
pub struct NewLetter<'a> {
    pub author: &'a str,
    pub message: &'a str,
    pub secret: bool,
}
