use super::Letter;
use crate::models::prelude::*;

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
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub letter: Letter,
    #[serde(rename = "type")]
    pub type_: i32,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(belongs_to(Letter))]
pub struct Report {
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub letter_id: Uuid,
    #[serde(rename = "type")]
    pub type_: i32,
    pub details: String,
    pub resolved: bool,
}
