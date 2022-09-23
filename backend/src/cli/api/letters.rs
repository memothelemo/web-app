use std::sync::Arc;

use actix_web::{error, web, Error, HttpResponse, Responder};
use backend_lib::db::{self, DbPool};

use serde::Deserialize;
use serde_json::json;

#[macro_export]
macro_rules! test_contraints {
    ($fn:ident, $expr:expr, $too_big:expr, $too_small:expr) => {
        if let Some(result) = $fn($expr) {
            return Err(error::ErrorConflict(json!({
                "message": match result {
                    crate::api::TestResult::TooFew => $too_small,
                    crate::api::TestResult::TooBig => $too_big,
                }
            })));
        }
    };
}

#[macro_export]
macro_rules! create_test_fn {
    ($name:ident, $max:expr, $min:expr) => {
        fn $name(text: &str) -> Option<crate::api::TestResult> {
            let author_len = text.len();
            if author_len > $max {
                Some(crate::api::TestResult::TooBig)
            } else if author_len < $min {
                Some(crate::api::TestResult::TooFew)
            } else {
                None
            }
        }
    };
    ($name:ident, $max:expr) => {
        fn $name(text: &str) -> Option<crate::api::TestResult> {
            let author_len = text.len();
            if author_len > $max {
                Some(crate::api::TestResult::TooBig)
            } else if author_len == 0 {
                Some(crate::api::TestResult::TooFew)
            } else {
                None
            }
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct GetPublicQuery {
    pub offset: Option<usize>,
}

#[actix_web::get("/api/letters")]
pub async fn get_public(
    query: web::Query<GetPublicQuery>,
    pool: DbPool,
) -> Result<impl Responder, Error> {
    let offset = query.offset.unwrap_or_default();
    let result = web::block(move || {
        let mut conn = pool.get()?;
        db::letters::get_all_public(&mut conn, offset)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Deserialize)]
pub struct PostLetterForm {
    pub author: String,
    pub message: String,
    pub secret: bool,
}

#[actix_web::post("/api/letters/post")]
pub async fn post(pool: DbPool, form: web::Json<PostLetterForm>) -> Result<impl Responder, Error> {
    log::info!("[post] user requested to post a letter");

    // check if we still accept submissions
    let pool_1 = pool.clone();
    let available = web::block(move || {
        let mut conn = pool.get()?;
        db::state::is_available(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if !available {
        return Err(error::ErrorUnauthorized(json!({
            "message": "cannot accept any new submissions"
        })));
    }

    // validate also to the server and sanitize dirty HTML
    // stuff to avoid XSS
    let author = form
        .author
        .trim()
        .chars()
        .filter(|v| !v.is_whitespace())
        .collect::<String>();

    let org_auth_len = form.author.len();
    let org_msg_len = form.message.len();
    let author = ammonia::clean(&author);

    if org_auth_len > author.len() {
        log::warn!(
            "[post] unexpected text from author field! original = {} > cleaned = {}",
            org_auth_len,
            author.len()
        );
    }

    static MIN_MESSAGE_LEN: usize = 50;
    static MAX_MESSAGE_LEN: usize = 1000;

    static MAX_AUTHOR_LEN: usize = 50;

    create_test_fn!(test_author, MAX_AUTHOR_LEN);
    create_test_fn!(test_message, MAX_MESSAGE_LEN, MIN_MESSAGE_LEN);

    test_contraints!(test_author, &author, "author too big", "author too small");

    // checking for existing letters with the author
    let pool = pool_1.clone();

    let author = Arc::new(author);
    let author_1 = author.clone();
    let letter = web::block(move || {
        let mut conn = pool_1.get()?;
        db::letters::find_by_author(&mut conn, &*author_1)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if letter.is_some() {
        return Err(error::ErrorConflict(json!({
            "message": "duplicated letter",
        })));
    }

    let message = ammonia::clean(&form.message);
    if org_msg_len > message.len() {
        log::warn!(
            "[post] XSS detected (message)! original = {} > cleaned = {}",
            org_msg_len,
            message.len()
        );
    }

    test_contraints!(
        test_message,
        &message,
        "message too big",
        "message too small"
    );
    drop(letter);

    let letter = web::block(move || {
        let mut conn = pool.get()?;
        db::letters::insert(&mut conn, &*author, message, form.secret)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(json!({
        "id": letter.id,
        "created_at": letter.created_at,
    })))
}
