mod api {
    use rocket::http::{ContentType, Status};
    use rocket::request::Outcome;
    use rocket::response::{Responder, Response};

    use serde_json::json;

    use std::borrow::Cow;
    use std::fmt::Display;
    use std::io::Cursor;

    #[derive(Debug)]
    pub struct ApiError<'a> {
        message: Cow<'a, str>,
        status: Status,
    }

    impl<'a> ApiError<'a> {
        pub fn borrowed(message: &'a str) -> Self {
            Self {
                message: Cow::Borrowed(message),
                status: Status::BadRequest,
            }
        }

        pub fn owned(message: impl Display) -> Self {
            Self {
                message: Cow::Owned(message.to_string()),
                status: Status::BadRequest,
            }
        }
    }

    impl<'a> ApiError<'a> {
        pub fn status(self, status: Status) -> Self {
            Self { status, ..self }
        }
    }

    impl<'a, E> From<E> for ApiError<'a>
    where
        E: Into<anyhow::Error>,
    {
        fn from(error: E) -> Self {
            Self {
                message: Cow::Owned(format!("{}", error.into())),
                status: Status::InternalServerError,
            }
        }
    }

    impl<'a, S, E> From<ApiError<'a>> for Outcome<S, E>
    where
        E: From<ApiError<'a>>,
    {
        fn from(err: ApiError<'a>) -> Self {
            Outcome::Failure((err.status, err.into()))
        }
    }

    impl<'a, 'r> Responder<'r, 'static> for ApiError<'a> {
        fn respond_to(
            self,
            _request: &'r rocket::Request<'_>,
        ) -> rocket::response::Result<'static> {
            // if let Some(accept) = request.accept() {
            //     static UNSUPPORTED_ENCODING: &str = "JSON encoding is required to use this API.";

            //     let accepts_json = accepts_media_type(accept, |c| {
            //         dbg!(c);
            //         c.is_json()
            //     });
            //     if !accepts_json {
            //         return Response::build()
            //             .sized_body(
            //                 UNSUPPORTED_ENCODING.len(),
            //                 Cursor::new(UNSUPPORTED_ENCODING),
            //             )
            //             .header(ContentType::Text)
            //             .status(Status::UnsupportedMediaType)
            //             .ok();
            //     }
            // }

            let output = serde_json::to_string(&json!({
                "message": self.message,
            }))
            .unwrap();

            Response::build()
                .sized_body(output.len(), Cursor::new(output))
                .header(ContentType::new("application", "json"))
                .status(self.status)
                .ok()
        }
    }
}
pub use api::*;
