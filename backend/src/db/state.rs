use crate::models;

use anyhow::Result;
use diesel::prelude::*;

pub fn is_available(conn: &mut PgConnection) -> Result<bool> {
    use crate::schema::states::dsl::*;

    log::info!("fetching state");
    let value = states
        .first::<models::State>(conn)
        .optional()?
        .map(|v| v.available)
        .unwrap_or_default();

    Ok(value)
}
