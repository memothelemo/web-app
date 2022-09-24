use crate::models::prelude::*;

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
pub struct State {
    pub id: i32,
    pub available: bool,
}
