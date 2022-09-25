use std::sync::Arc;

use actix_web::web::ServiceConfig;
use actix_web::{error, web, Error, HttpResponse, Responder};

use backend_lib::db::{self, DbPool};
use backend_lib::models;
use backend_lib::reqs::user::UserRestrictions;

use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{create_test_fn, test_contraints};

#[derive(Debug, Deserialize)]
pub struct RetrieveLetterQuery {
    offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ReportActionQuery {
    id: Uuid,
}

#[actix_web::delete("/api/reports/resolve")]
pub async fn resolve(
    restrictions: UserRestrictions,
    pool: DbPool,
    query: web::Query<ReportActionQuery>,
) -> Result<impl Responder, Error> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden(json!({
            "message": "not authorized to revoke reports",
        })))
    } else {
        let pool = Arc::new(pool);
        let pool_1 = pool.clone();

        let id = query.id.clone();

        let report = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get(&mut conn, id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        let report = match report {
            Some(n) => n,
            None => {
                return Err(error::ErrorNotFound(json!({
                    "message": "report not found",
                })))
            }
        };

        // delete the report first before the letter
        // otherwise we can get an error from Postgres
        let pool = pool_1.clone();
        web::block(move || {
            let mut conn = pool_1.get()?;
            db::reports::delete(&mut conn, query.id)
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
            "message": "report resolved",
        })))
    }
}

#[actix_web::delete("/api/reports/revoke/{id}")]
pub async fn revoke(
    restrictions: UserRestrictions,
    pool: DbPool,
    query: web::Query<ReportActionQuery>,
) -> Result<impl Responder, Error> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden(json!({
            "message": "not authorized to revoke reports",
        })))
    } else {
        let pool = Arc::new(pool);
        let pool_1 = pool.clone();

        let id = query.id.clone();

        let report = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get(&mut conn, id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        if report.is_none() {
            return Err(error::ErrorNotFound(json!({
                "message": "report not found",
            })));
        }

        web::block(move || {
            let mut conn = pool_1.get()?;
            db::reports::delete(&mut conn, query.id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Accepted().json(json!({
            "message": "report revoked",
        })))
    }
}

#[actix_web::get("/api/reports/letters")]
pub async fn get_pending_letters(
    restrictions: UserRestrictions,
    pool: DbPool,
    query: web::Query<RetrieveLetterQuery>,
) -> Result<impl Responder, Error> {
    if !restrictions.moderator {
        Err(error::ErrorForbidden(json!({
            "message": "not authorized to view reports",
        })))
    } else {
        let letters = web::block(move || {
            let mut conn = pool.get()?;
            db::reports::get_all_pending(&mut conn, query.offset.unwrap_or_default())
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(letters))
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
) -> Result<impl Responder, Error> {
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
        return Err(error::ErrorConflict(json!({
            "message": "attempt to report a non-existing letter",
        })));
    }

    log::debug!("[letter] existing letter id = {:?}", uuid);

    static MIN_EMAIL_LEN: usize = 3;
    static MAX_EMAIL_LEN: usize = 50;

    static MAX_DETAILS_LEN: usize = 1000;

    let email = ammonia::clean(&form.email);
    let details = ammonia::clean(&form.details);

    create_test_fn!(test_email, MAX_DETAILS_LEN, MIN_EMAIL_LEN);
    create_test_fn!(test_details, MAX_DETAILS_LEN);

    test_contraints!(test_email, &email, "email too small", "email too big");
    test_contraints!(
        test_details,
        &details,
        "details too small",
        "details too big"
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
    cfg.service(get_pending_letters).service(report_letter);
}
