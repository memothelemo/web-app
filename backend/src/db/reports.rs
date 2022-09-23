use crate::models;

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_all_pending(
    conn: &mut PgConnection,
    offset: usize,
) -> Result<Vec<models::PendingReport>> {
    use crate::schema::reports::dsl::*;

    log::info!("[get_all_pending] fetching (offset = {})", offset);
    let collection = reports
        .filter(resolved.eq(false))
        .select((id, email, created_at, letter_id, type_, details))
        .offset(offset.max(0) as i64)
        .limit(50)
        .load::<models::PendingReport>(conn)?;

    Ok(collection)
}

pub fn insert(
    conn: &mut PgConnection,
    entry_letter_id: Uuid,
    entry_email: impl AsRef<str>,
    entry_details: impl AsRef<str>,
    entry_type: models::ReportType,
) -> Result<models::Report> {
    let entry_email = entry_email.as_ref();
    let entry_details = entry_details.as_ref();

    log::info!("[insert] posting report");
    log::info!("[insert] email = {:?}", entry_email);
    log::info!("[insert] type = {:?}", entry_type);

    use crate::schema::reports::dsl::*;

    let new_report = models::NewReport {
        email: entry_email,
        letter_id: entry_letter_id,
        type_: entry_type as i32,
        details: entry_details,
    };

    Ok(diesel::insert_into(reports)
        .values(&new_report)
        .get_result(conn)?)
}
