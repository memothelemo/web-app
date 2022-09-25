use actix_web::{error, web, FromRequest};

use serde_json::json;
use uuid::Uuid;

use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

use crate::config::AuthParams;
use crate::db::{self, DbPool};
use crate::models::UserToken;

/// Limitations of an individual user authenticated
#[derive(Debug)]
pub struct UserRestrictions {
    pub moderator: bool,
    pub viewer: bool,
}

static ACTIX_DATA_NOT_CONFIGURED: &str = "Requested application data is not configured correctly. \
View/enable debug logs for more details.";

impl FromRequest for UserRestrictions {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let params = match req.app_data::<web::Data<AuthParams>>() {
            Some(n) => n,
            None => {
                // stolen from actix-web :)
                log::debug!("[UserRestrictions] Failed to extract Data<AuthParams>");
                return Box::pin(async {
                    Err(error::ErrorInternalServerError(ACTIX_DATA_NOT_CONFIGURED))
                });
            }
        };

        let auth = if let Some(auth) = req.headers().get("authorization") {
            if let Ok(auth) = auth.to_str() {
                if let Some(auth) = auth.strip_prefix("Bearer ") {
                    Some(UserToken::decode_token(&auth, &params.token).map_err(|e| {
                        log::error!("[UserRestrictions] failed to decode token: {}", e);
                        error::ErrorInternalServerError("Failed to evaluate token")
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let token_data = match auth {
            Some(Ok(n)) => n,
            Some(Err(err)) => return Box::pin(async { Err(err) }),
            None => {
                return Box::pin(async {
                    Err(error::ErrorUnauthorized(json!({
                        "message": "unauthorized",
                    })))
                })
            }
        };

        // get the user from the sub field
        let pool = match req.app_data::<DbPool>() {
            Some(n) => n,
            None => {
                log::debug!("[UserRestrictions] Failed to extract Data<DbPool>");
                return Box::pin(async {
                    Err(error::ErrorInternalServerError(ACTIX_DATA_NOT_CONFIGURED))
                });
            }
        };

        let mut pool = match pool.get() {
            Ok(n) => n,
            Err(err) => {
                log::error!("[UserRestrictions] db error: {}", err);
                return Box::pin(async {
                    Err(error::ErrorInternalServerError(
                        error::ErrorInternalServerError("Failed to evaluate token"),
                    ))
                });
            }
        };

        let uuid = match Uuid::from_str(&token_data.claims.sub) {
            Ok(n) => n,
            Err(err) => {
                log::error!("[UserRestrictions] invalid user id: {}", err);
                return Box::pin(async {
                    Err(error::ErrorInternalServerError(
                        error::ErrorInternalServerError("Failed to evaluate token"),
                    ))
                });
            }
        };

        let user = match db::users::find_by_id(&mut pool, &uuid) {
            Ok(user) => user,
            Err(err) => {
                log::error!("[UserRestrictions] db error: {}", err);
                return Box::pin(async {
                    Err(error::ErrorInternalServerError(
                        error::ErrorInternalServerError("Failed to evaluate token"),
                    ))
                });
            }
        };

        if let Some(user) = user {
            Box::pin(async move {
                Ok(UserRestrictions {
                    moderator: user.moderator.unwrap_or_default(),
                    viewer: user.viewer.unwrap_or_default(),
                })
            })
        } else {
            Box::pin(async {
                Err(error::ErrorUnauthorized(json!({
                    "message": "unauthorized",
                })))
            })
        }
    }
}
