use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ReportType {
    Spam = 1,
    Abuse = 2,
    BugReport = 3,
    TechnicalIssue = 4,
    Profanity = 5,
    Bullying = 6,
    InapproriateContent = 7,
    NSFW = 8,
    Scam = 9,
    Irrelevant = 10,
    Others = 11,
}

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = reports)]
pub struct NewReport<'a> {
    pub email: &'a str,
    pub letter_id: Uuid,
    #[serde(rename = "type")]
    pub type_: i32,
    pub details: &'a str,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[diesel(belongs_to(Letter))]
pub struct PendingReport {
    pub id: i64,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub letter_id: Uuid,
    #[serde(rename = "type")]
    pub type_: i32,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(belongs_to(Letter))]
pub struct Report {
    pub id: i64,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub letter_id: Uuid,
    #[serde(rename = "type")]
    pub type_: i32,
    pub details: String,
    pub resolved: bool,
}

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

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
pub struct State {
    pub id: i32,
    pub available: bool,
}
