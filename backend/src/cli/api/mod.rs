use actix_web::{error, web, Error, HttpResponse, Responder};
use backend_lib::db::{self, DbPool};
use serde_json::json;

pub mod letters;
pub mod reports;

pub enum TestResult {
    TooFew,
    TooBig,
}

#[actix_web::get("/api")]
pub async fn index() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "API is running!",
    })))
}

#[actix_web::get("/api/available")]
pub async fn is_available(pool: DbPool) -> Result<impl Responder, Error> {
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
