use backend_lib::db::submission::SubmissionQuery;
use backend_lib::db::{DbClient, Queryable};
use backend_lib::errors::ApiError;

use rocket::serde::json::Json;
use rocket::{catchers, routes, State};

use rocket_governor::rocket_governor_catcher;

use serde_json::json;

type JsonResponse<'a> = Result<Json<serde_json::Value>, ApiError<'a>>;
type RocketBuild = rocket::Rocket<rocket::Build>;

pub mod forms {
    use super::*;

    use backend_lib::db::letters::{CreateLetterQuery, GetLetterQuery};
    use backend_lib::db::user::GetUserQuery;
    use backend_lib::db::MaybeQueryable;
    use backend_lib::restrictions::RateLimit;

    use rocket::form::Form;
    use rocket::http::Status;
    use rocket::FromForm;
    use serde::Deserialize;

    #[derive(FromForm)]
    pub struct LoginForm<'a> {
        pub username: &'a str,
        pub password: &'a str,
    }

    #[rocket::post("/login", data = "<form>")]
    pub async fn login(
        _guard: RateLimit<'_>,
        db: &State<DbClient>,
        form: Form<LoginForm<'_>>,
    ) -> JsonResponse<'static> {
        // we want to make errors vague as possible to avoid
        // possible hacking scenarios which we don't want to do that
        macro_rules! login_err {
            () => {
                Err(ApiError::owned("Invalid credientals!")
                    .status(Status::Forbidden)
                    .into())
            };
        }

        if form.username.is_empty() || form.password.is_empty() {
            return login_err!();
        }

        let mut authenticated = false;
        let user = GetUserQuery::Username(form.username).query(&db).await?;

        if let Some(user) = user {
            authenticated = user.password == form.password;
        }

        if authenticated {
            Ok(Json(json!({ "message": "Granted!" })))
        } else {
            login_err!()
        }
    }

    #[derive(Deserialize, FromForm)]
    pub struct PostLetterForm<'a> {
        #[serde(borrow)]
        pub author: &'a str,
        #[serde(borrow)]
        pub message: &'a str,
        #[serde(default)]
        pub secret: bool,
    }

    async fn post_letter_inner(
        db: &State<DbClient>,
        form: Json<PostLetterForm<'_>>,
    ) -> JsonResponse<'static> {
        // validate also to the server and sanitize dirty HTML
        // stuff to avoid XSS
        let org_auth_len = form.author.len();
        let org_msg_len = form.message.len();
        let author = ammonia::clean(form.author);
        if org_auth_len > author.len() {
            log::warn!(
                "[api/letters/post] XSS detected (author)! original = {} > cleaned = {}",
                org_auth_len,
                author.len()
            );
        }
        if author.is_empty() {
            return Err(ApiError::borrowed("author too short")
                .status(Status::BadRequest)
                .into());
        }
        let letter = GetLetterQuery::Author(form.author).query(&db).await?;
        if letter.is_some() {
            return Err(ApiError::borrowed("duplicated letter")
                .status(Status::Conflict)
                .into());
        }
        let message = ammonia::clean(form.message);
        if org_msg_len > message.len() {
            log::warn!(
                "[api/letters/post] XSS detected (message)! original = {} > cleaned = {}",
                org_msg_len,
                message.len()
            );
        }
        if message.len() < 50 {
            return Err(ApiError::borrowed("message too short")
                .status(Status::BadRequest)
                .into());
        }
        if message.len() > 1000 {
            return Err(ApiError::borrowed("message too big")
                .status(Status::BadRequest)
                .into());
        }
        Ok(Json(serde_json::to_value(
            CreateLetterQuery::new(&author, &message)
                .secret(form.secret)
                .query(&db)
                .await?,
        )?))
    }

    #[cfg(not(test))]
    #[rocket::post("/letters/post", data = "<form>")]
    pub async fn post_letter(
        _guard: RateLimit<'_>,
        db: &State<DbClient>,
        form: Json<PostLetterForm<'_>>,
    ) -> JsonResponse<'static> {
        post_letter_inner(db, form).await
    }

    #[cfg(test)]
    #[rocket::post("/letters/post", data = "<form>")]
    pub async fn post_letter(
        db: &State<DbClient>,
        form: Json<PostLetterForm<'_>>,
    ) -> JsonResponse<'static> {
        post_letter_inner(db, form).await
    }
}

#[rocket::get("/")]
pub async fn root() -> JsonResponse<'static> {
    Ok(Json(json!({
        "message": "API is running!",
    })))
}

mod letters {
    use super::*;
    use backend_lib::{db::letters::GetPublicLettersQuery, restrictions::RateLimit};

    #[rocket::get("/public")]
    pub async fn public(_guard: RateLimit<'_>, db: &State<DbClient>) -> JsonResponse<'static> {
        Ok(Json(
            serde_json::to_value(GetPublicLettersQuery.query(db).await?).unwrap(),
        ))
    }
}

#[rocket::get("/available")]
pub async fn available(db: &State<DbClient>) -> JsonResponse {
    let state = SubmissionQuery.query(db).await?;
    Ok(Json(json!({
        "available": state.available,
    })))
}

pub fn apply(rocket: RocketBuild) -> RocketBuild {
    rocket
        .mount("/api", routes![root, available])
        .mount("/api", routes![forms::login, forms::post_letter])
        .mount("/api/letters", routes![letters::public])
        .register("/api", catchers!(rocket_governor_catcher))
}
