use std::sync::Arc;

use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;

use actix_web::web::{self, ServiceConfig};
use actix_web::{error, Error, HttpResponse, Responder};

use anyhow::Context;

use backend_lib::config::AuthParams;
use backend_lib::db::{self, DbPool};
use backend_lib::models::{NewUser, UserToken, TOKEN_EXPIRY_DURATION};
use backend_lib::reqs::register::RegisterAuth;

use serde::Deserialize;
use serde_json::json;

use crate::{create_test_fn, test_contraints};

#[derive(Debug, Deserialize)]
pub struct UserForm {
    username: String,
    password: String,
}

static MAX_USERNAME_LEN: usize = 50;

static MIN_PASSWORD_LEN: usize = 12;
static MAX_PASSWORD_LEN: usize = 50;

static INVALID_CREDIENTALS: &str = "invalid credientials";

create_test_fn!(test_password, MAX_PASSWORD_LEN, MIN_PASSWORD_LEN);
create_test_fn!(test_username, MAX_USERNAME_LEN);

#[actix_web::post("/api/users/register")]
pub async fn register(
    _authorized: RegisterAuth,
    pool: DbPool,
    auth_params: web::Data<AuthParams>,
    form: web::Json<UserForm>,
) -> Result<impl Responder, Error> {
    test_contraints!(
        test_username,
        &form.username,
        "username too short",
        "username too long"
    );
    test_contraints!(
        test_password,
        &form.password,
        "password too short",
        "password too long"
    );

    // check for any duplications
    let form = Arc::new(form);
    let form_1 = form.clone();

    let pool = Arc::new(pool);
    let pool_1 = pool.clone();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::users::find_by_username(&mut conn, &*form_1.username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if user.is_some() {
        return Err(error::ErrorConflict(json!({
            "message": "Already registered!",
        })));
    }

    let formed_password = format!("{}{:?}{}", form.username, auth_params.salt, form.password);

    // encrypt the password and add some salting into it
    // hackers will happy to investigate because of AGPL license
    let parts = bcrypt::hash_with_salt(&formed_password, 10, auth_params.salt.clone())
        .with_context(|| "failed to hash password")
        .map_err(|e| {
            log::error!("[register] failed to generate password hash: {}", e);
            error::ErrorInternalServerError(
                "There's something wrong to our server, please try again later",
            )
        })?;

    let user = web::block(move || {
        let mut conn = pool_1.get()?;
        db::users::insert(
            &mut conn,
            NewUser {
                name: &form.username,
                password: &parts.to_string(),
            },
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(json!({
        "id": user.id,
        "created_at": user.created_at,
    })))
}

#[actix_web::post("/api/users/login")]
pub async fn login(
    pool: DbPool,
    auth_params: web::Data<AuthParams>,
    form: web::Json<UserForm>,
) -> Result<impl Responder, Error> {
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

    let formed_password = format!("{}{:?}{}", form.username, auth_params.salt, form.password);
    if !bcrypt::verify(&formed_password, &user.password).unwrap_or_default() {
        return Err(error::ErrorUnauthorized(json!({
            "message": INVALID_CREDIENTALS,
        })));
    }

    let token =
        UserToken::generate_token(&user.id.to_string(), &auth_params.token).map_err(|e| {
            log::error!("[login] failed to generate jwt token: {}", e);
            error::ErrorInternalServerError(
                "There's something wrong to our server, please try again later",
            )
        })?;

    let mut response = HttpResponse::Accepted().json(json!({
        "message": "Logged in!",
    }));

    response
        .add_cookie(
            &Cookie::build("token", token)
                .max_age(Duration::seconds(TOKEN_EXPIRY_DURATION))
                .finish(),
        )
        .map_err(error::ErrorInternalServerError)?;

    Ok(response)
}

pub fn apply(cfg: &mut ServiceConfig) {
    cfg.service(login).service(register);
}
