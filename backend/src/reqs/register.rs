use actix_web::{error, FromRequest};
use lazy_static::lazy_static;

use serde_json::json;

use std::future::Future;
use std::pin::Pin;

lazy_static! {
    pub static ref REGISTER_KEY: String =
        std::env::var("REGISTER_KEY").expect("REGISTER_KEY is not present");
}

pub struct RegisterAuth;

impl FromRequest for RegisterAuth {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let mut authorized = false;
        if let Some(auth) = req.headers().get("authorization") {
            if let Ok(auth) = auth.to_str() {
                if let Some(auth) = auth.strip_prefix("Bearer ") {
                    authorized = auth == REGISTER_KEY.as_str();
                }
            }
        }

        Box::pin(async move {
            if authorized {
                Ok(Self)
            } else {
                Err(error::ErrorUnauthorized("Not authorized"))
            }
        })
    }
}
