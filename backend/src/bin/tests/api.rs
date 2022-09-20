#[cfg(feature = "test_log")]
use env_logger::Env;

use faker_rand::en_us::names::FullName;
use faker_rand::lorem::Paragraph;

use pretty_assertions::assert_eq;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::json;

fn client() -> Client {
    #[cfg(feature = "test_log")]
    env_logger::builder()
        .parse_env(Env::new().default_filter_or("debug"))
        .try_init()
        .ok();

    Client::tracked(crate::server()).expect("valid rocket instance")
}

#[test]
fn root() {
    let client = client();
    let response = client.get("/api").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json(),
        Some(json!({
            "message": "API is running!",
        }))
    );
}

#[test]
fn available() {
    let client = client();
    let response = client.get("/api/available").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json(),
        Some(json!({
            "available": true,
        }))
    );
}

#[test]
fn create_letter() {
    let author = rand::random::<FullName>().to_string();
    let message = rand::random::<Paragraph>().to_string();
    let secret = false;

    let client = client();
    let response = client
        .post("/api/letters/post")
        .header(ContentType::JSON)
        .body(
            serde_json::to_string(&json!({
                "message": message,
                "author": author,
                "secret": secret,
            }))
            .unwrap(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    serde_json::from_str::<backend_lib::schema::Letter>(&response.into_string().unwrap()).unwrap();

    // duplication test
    let response = client
        .post("/api/letters/post")
        .header(ContentType::JSON)
        .body(
            serde_json::to_string(&json!({
                "message": message,
                "author": author,
                "secret": secret,
            }))
            .unwrap(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(
        response.into_json(),
        Some(json!({
            "message": "duplicated letter"
        }))
    );

    // message contraints
    let response = client
        .post("/api/letters/post")
        .header(ContentType::JSON)
        .body(
            serde_json::to_string(&json!({
                "message": "foo",
                "author": "memo",
                "secret": false,
            }))
            .unwrap(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
        response.into_json(),
        Some(json!({
            "message": "message too short"
        }))
    );

    let response = client
        .post("/api/letters/post")
        .header(ContentType::JSON)
        .body(
            serde_json::to_string(&json!({
                "message": message,
                "author": "",
                "secret": false,
            }))
            .unwrap(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
        response.into_json(),
        Some(json!({
            "message": "author too short"
        }))
    );
}
