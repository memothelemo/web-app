use std::sync::Arc;

use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};

use backend_lib::config::AuthParams;
use backend_lib::db::{self, DbPool};
use backend_lib::models;

use backend_lib::reqs::user::UserRestrictions;
use backend_lib::resp::error::{self, ApiError};

use backend_lib::utils::letter::decrypt_message;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{create_test_fn, test_contraints};

#[derive(Debug, Deserialize)]
pub struct RetrieveLetterQuery {
    offset: Option<usize>,
}

#[actix_web::delete("/api/reports/resolve/{id}")]
pub async fn resolve(
    restrictions: UserRestrictions,
    pool: DbPool,
    id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden("Not authorized to revoke reports"))
    } else {
        let pool = Arc::new(pool);
        let pool_1 = pool.clone();

        let id_1 = id.clone();

        let report = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get_pending(&mut conn, id_1)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        let report = match report {
            Some(n) => n,
            None => return Err(error::ErrorNotFound("Report not found")),
        };

        // delete the report first before the letter
        // otherwise we can get an error from Postgres
        let pool = pool_1.clone();
        web::block(move || {
            let mut conn = pool_1.get()?;
            db::reports::delete(&mut conn, id.into_inner())
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        web::block(move || {
            let mut conn = pool.get()?;
            db::letters::delete(&mut conn, report.id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Accepted().json(json!({
            "message": "Report resolved",
        })))
    }
}

#[actix_web::delete("/api/reports/revoke/{id}")]
pub async fn revoke(
    restrictions: UserRestrictions,
    pool: DbPool,
    id: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden("Not authorized to revoke reports"))
    } else {
        let pool = Arc::new(pool);
        let pool_1 = pool.clone();

        let id_1 = id.clone();

        let report = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get_pending(&mut conn, id_1)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        if report.is_none() {
            return Err(error::ErrorNotFound("Report not found"));
        }

        web::block(move || {
            let mut conn = pool_1.get()?;
            db::reports::delete(&mut conn, id.into_inner())
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Accepted().json(json!({
            "message": "Report revoked",
        })))
    }
}

#[actix_web::get("/api/reports/letters")]
pub async fn get_pending_letters(
    auth_params: web::Data<AuthParams>,
    restrictions: UserRestrictions,
    pool: DbPool,
    query: web::Query<RetrieveLetterQuery>,
) -> Result<impl Responder, ApiError> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden("Not authorized to view reports"))
    } else {
        let mut reports = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get_all_pending(&mut conn, query.offset.unwrap_or_default())
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        log::info!("[get_pending_letters] processing reports");
        let encrypted_reports = reports.iter_mut().filter(|v| v.letter.secret);
        for report in encrypted_reports {
            log::info!(
                "[get_pending_letters] secret letter found (id = {}); decrypting...",
                report.letter.id
            );
            let message = decrypt_message(
                &auth_params.secret_key,
                &report.letter.author,
                &report.letter.message,
            )
            .await
            .map_err(|e| {
                log::error!(
                    "[get_pending_letters] failed to decrypt report letter message (id = {:?}): {}",
                    report.letter.id,
                    e
                );
                ApiError::we_pretend_why_it_does_error()
            })?;
            log::info!(
                "[get_pending_letters] done decrypting report letter = {}",
                report.letter.id
            );
            report.letter.message = message;
        }

        Ok(HttpResponse::Ok().json(reports))
    }
}

#[derive(Deserialize)]
pub struct ReportLetterForm {
    pub email: String,
    #[serde(rename = "type")]
    pub type_: models::ReportType,
    pub details: String,
}

#[actix_web::post("/api/letters/report/{uuid}")]
pub async fn report_letter(
    letter_uid: web::Path<Uuid>,
    pool: DbPool,
    form: web::Json<ReportLetterForm>,
) -> Result<impl Responder, ApiError> {
    let pool = Arc::new(pool);
    let pool_1 = pool.clone();

    let uuid = Arc::new(letter_uid.into_inner());
    let uuid_1 = uuid.clone();

    let letter = web::block(move || {
        let mut conn = pool_1.get()?;
        db::letters::find_by_id(&mut conn, &*uuid_1)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if letter.is_none() {
        log::warn!("[letter] non-existing letter id = {:?}", uuid);
        return Err(error::ErrorConflict(
            "Attempt to report a non-existing letter",
        ));
    }

    log::debug!("[letter] existing letter id = {:?}", uuid);

    static MIN_EMAIL_LEN: usize = 3;
    static MAX_EMAIL_LEN: usize = 50;

    static MAX_DETAILS_LEN: usize = 1000;

    let email = ammonia::clean(&form.email);
    let details = ammonia::clean(&form.details);

    create_test_fn!(test_email, MAX_DETAILS_LEN, MIN_EMAIL_LEN);
    create_test_fn!(test_details, MAX_DETAILS_LEN);

    test_contraints!(
        test_email,
        &email,
        "Email field is too big",
        "Email field is too small"
    );
    test_contraints!(
        test_details,
        &details,
        "Details field is too big",
        "Details field is too small"
    );

    let report = web::block(move || {
        let mut conn = pool.get()?;
        db::reports::insert(&mut conn, *uuid, &form.email, &form.details, form.type_)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(json!({
        "id": report.id,
        "created_at": report.created_at,
    })))
}

pub fn apply(cfg: &mut ServiceConfig) {
    cfg.service(resolve)
        .service(revoke)
        .service(get_pending_letters)
        .service(report_letter);
}
