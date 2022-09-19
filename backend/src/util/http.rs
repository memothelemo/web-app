use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};

use mime::Mime;
use rocket::http::{Accept, MediaType};

pub fn accepts_media_type(accept: &Accept, mut condition: impl FnMut(&MediaType) -> bool) -> bool {
    for mdt in accept.media_types() {
        if condition(mdt) {
            return true;
        }
    }
    false
}

pub fn get_content_type(headers: &HeaderMap<HeaderValue>) -> Option<Mime> {
    if let Some(header) = headers.get(header::CONTENT_TYPE) {
        if let Ok(header) = header.to_str() {
            use std::str::FromStr;
            return Mime::from_str(header).ok();
        }
    }
    None
}
