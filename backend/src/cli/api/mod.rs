use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpResponse, Responder};

use backend_lib::db::{self, DbPool};
use backend_lib::resp::error::{self, ApiError};

use serde_json::json;

pub mod letters;
pub mod reports;
pub mod users;

pub enum TestResult {
    TooFew,
    TooBig,
}

#[actix_web::get("/api")]
pub async fn index() -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "API is running!",
    })))
}

#[actix_web::get("/api/available")]
pub async fn is_available(pool: DbPool) -> Result<impl Responder, ApiError> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        db::state::is_available(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "available": result,
    })))
}

pub fn apply(cfg: &mut ServiceConfig) {
    cfg.configure(letters::apply)
        .configure(users::apply)
        .configure(reports::apply)
        .service(index)
        .service(is_available);
}
