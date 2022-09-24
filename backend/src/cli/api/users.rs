use std::sync::Arc;

use actix_web::web::{self, ServiceConfig};
use actix_web::{error, Error, HttpResponse, Responder};

use backend_lib::db::{self, DbPool};

use serde::Deserialize;
use serde_json::json;

use crate::{create_test_fn, test_contraints};

#[derive(Debug, Deserialize)]
pub struct UserLoginForm {
    username: String,
    password: String,
}

static MIN_PASSWORD_LEN: usize = 8;
static MAX_PASSWORD_LEN: usize = 50;

#[actix_web::post("/api/users/login")]
pub async fn login(pool: DbPool, form: web::Json<UserLoginForm>) -> Result<impl Responder, Error> {
    static INVALID_CREDIENTALS: &str = "invalid credientials";

    create_test_fn!(test_password, MAX_PASSWORD_LEN, MIN_PASSWORD_LEN);
    test_contraints!(
        test_password,
        &form.password,
        INVALID_CREDIENTALS,
        INVALID_CREDIENTALS
    );

    let form = Arc::new(form);
    let form_1 = form.clone();

    let pool = Arc::new(pool);
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::users::find_by_username(&mut conn, &*form_1.username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let user = match user {
        Some(n) => n,
        None => {
            return Err(error::ErrorUnauthorized(json!({
                "message": INVALID_CREDIENTALS,
            })))
        }
    };

    if !bcrypt::verify(&form.password, &user.password).unwrap_or_default() {
        return Err(error::ErrorUnauthorized(json!({
            "message": INVALID_CREDIENTALS,
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": "Logged in!",
    })))
}

pub fn apply(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
