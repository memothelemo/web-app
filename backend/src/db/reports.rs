use crate::models;

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete(conn: &mut PgConnection, report_id: Uuid) -> Result<()> {
    log::info!("[delete] id = {}", report_id);
    use crate::schema::reports::dsl::*;

    diesel::delete(reports.filter(id.eq(report_id))).execute(conn)?;
    Ok(())
}

pub fn get_pending(
    conn: &mut PgConnection,
    report_id: Uuid,
) -> Result<Option<models::PendingReport>> {
    log::info!("[get] id = {}", report_id);

    use crate::schema::letters::dsl as dsl_letter;
    use crate::schema::reports::dsl::*;

    let report = reports
        .filter(id.eq(report_id))
        .first::<models::Report>(conn)
        .optional()?;

    if let Some(report) = report {
        let letter = dsl_letter::letters
            .filter(dsl_letter::id.eq(report.letter_id))
            .first::<models::Letter>(conn)?;

        Ok(Some(models::PendingReport {
            id: report.id,
            email: report.email,
            created_at: report.created_at,
            letter,
            type_: report.type_,
            details: report.details,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_all_pending(
    conn: &mut PgConnection,
    offset: usize,
) -> Result<Vec<models::PendingReport>> {
    use crate::schema::letters;
    use crate::schema::reports::dsl::*;

    log::info!("[get_all_pending] fetching (offset = {})", offset);
    let collection: Vec<(models::Report, models::Letter)> = reports
        .filter(resolved.eq(false))
        .inner_join(letters::table)
        .offset(offset.max(0) as i64)
        .limit(10)
        .load::<(models::Report, models::Letter)>(conn)?;

    Ok(collection
        .into_iter()
        .map(|(report, letter)| models::PendingReport {
            id: report.id,
            email: report.email,
            created_at: report.created_at,
            letter,
            type_: report.type_,
            details: report.details,
        })
        .collect::<Vec<_>>())
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
