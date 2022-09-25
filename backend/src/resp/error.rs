use actix_web::body::BoxBody;

use actix_web::http::header::{self, TryIntoHeaderValue};
use actix_web::http::StatusCode;

use actix_web::web::{BufMut, BytesMut};
use actix_web::{HttpResponse, ResponseError};

use std::fmt::Display;

use serde_json::json;

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(cause: impl Display, status: StatusCode) -> Self {
        Self {
            status,
            message: cause.to_string(),
        }
    }
}

impl ApiError {
    // lmao
    #[inline]
    pub fn we_pretend_why_it_does_error() -> Self {
        Self::new(
            "There's something wrong to our server, please try again later",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl<T> From<T> for ApiError
where
    T: Into<anyhow::Error>,
{
    fn from(error: T) -> Self {
        ErrorInternalServerError(error.into())
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status.clone()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status);
        let mut buf = BytesMut::new().writer();
        serde_json::to_writer(
            &mut buf,
            &json!({
                "message": self.message,
            }),
        )
        .unwrap();

        let mime = mime::APPLICATION_JSON.try_into_value().unwrap();
        res.headers_mut().insert(header::CONTENT_TYPE, mime);
        res.set_body(BoxBody::new(buf.into_inner()))
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.message, f)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

macro_rules! error_helper {
    ($name:ident, $status:ident) => {
        #[doc = concat!("Helper function that wraps any error and generates a `", stringify!($status), "` response.")]
        #[allow(non_snake_case)]
        pub fn $name<T>(err: T) -> ApiError
        where
            T: Display,
        {
            ApiError::new(err, StatusCode::$status)
        }
    };
}

error_helper!(ErrorBadRequest, BAD_REQUEST);
error_helper!(ErrorUnauthorized, UNAUTHORIZED);
error_helper!(ErrorPaymentRequired, PAYMENT_REQUIRED);
error_helper!(ErrorForbidden, FORBIDDEN);
error_helper!(ErrorNotFound, NOT_FOUND);
error_helper!(ErrorMethodNotAllowed, METHOD_NOT_ALLOWED);
error_helper!(ErrorNotAcceptable, NOT_ACCEPTABLE);
error_helper!(
    ErrorProxyAuthenticationRequired,
    PROXY_AUTHENTICATION_REQUIRED
);
error_helper!(ErrorRequestTimeout, REQUEST_TIMEOUT);
error_helper!(ErrorConflict, CONFLICT);
error_helper!(ErrorGone, GONE);
error_helper!(ErrorLengthRequired, LENGTH_REQUIRED);
error_helper!(ErrorPayloadTooLarge, PAYLOAD_TOO_LARGE);
error_helper!(ErrorUriTooLong, URI_TOO_LONG);
error_helper!(ErrorUnsupportedMediaType, UNSUPPORTED_MEDIA_TYPE);
error_helper!(ErrorRangeNotSatisfiable, RANGE_NOT_SATISFIABLE);
error_helper!(ErrorImATeapot, IM_A_TEAPOT);
error_helper!(ErrorMisdirectedRequest, MISDIRECTED_REQUEST);
error_helper!(ErrorUnprocessableEntity, UNPROCESSABLE_ENTITY);
error_helper!(ErrorLocked, LOCKED);
error_helper!(ErrorFailedDependency, FAILED_DEPENDENCY);
error_helper!(ErrorUpgradeRequired, UPGRADE_REQUIRED);
error_helper!(ErrorPreconditionFailed, PRECONDITION_FAILED);
error_helper!(ErrorPreconditionRequired, PRECONDITION_REQUIRED);
error_helper!(ErrorTooManyRequests, TOO_MANY_REQUESTS);
error_helper!(
    ErrorRequestHeaderFieldsTooLarge,
    REQUEST_HEADER_FIELDS_TOO_LARGE
);
error_helper!(
    ErrorUnavailableForLegalReasons,
    UNAVAILABLE_FOR_LEGAL_REASONS
);
error_helper!(ErrorExpectationFailed, EXPECTATION_FAILED);
error_helper!(ErrorInternalServerError, INTERNAL_SERVER_ERROR);
error_helper!(ErrorNotImplemented, NOT_IMPLEMENTED);
error_helper!(ErrorBadGateway, BAD_GATEWAY);
error_helper!(ErrorServiceUnavailable, SERVICE_UNAVAILABLE);
error_helper!(ErrorGatewayTimeout, GATEWAY_TIMEOUT);
error_helper!(ErrorHttpVersionNotSupported, HTTP_VERSION_NOT_SUPPORTED);
error_helper!(ErrorVariantAlsoNegotiates, VARIANT_ALSO_NEGOTIATES);
error_helper!(ErrorInsufficientStorage, INSUFFICIENT_STORAGE);
error_helper!(ErrorLoopDetected, LOOP_DETECTED);
error_helper!(ErrorNotExtended, NOT_EXTENDED);
error_helper!(
    ErrorNetworkAuthenticationRequired,
    NETWORK_AUTHENTICATION_REQUIRED
);
