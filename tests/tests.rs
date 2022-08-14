use lazy_static::lazy_static;
use rocket::http::{ContentType, Status};
use rust_graphql_backend::data::db::ResponseUser;
use serde_json;

mod common;

#[test]
fn hello_test() {
    let client = common::setup();
    let mut response = client.get("/hello/edwards/23").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("Hello, 23 year old named edwards!".into())
    );
}

#[test]
fn user_list_rt_test() {
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let mut response_body = response.body_string().unwrap();
    response_body.retain(|c| !c.is_numeric());
    assert_eq!(response_body, "[]");
}

#[test]
fn new_user_rt_test() {
    let client = common::setup();
    let mut response = client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(
            r##"{
        "name": "John Doe",
        "email": "j.doe@gmail.com",
        "password": "tester"
        }"##,
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = response.body_string().expect("Response Body");

    let user: ResponseUser =
        serde_json::from_str(&response_body.as_str()).expect("Valid User Response");

    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "j.doe@gmail.com");
}

#[test]
fn info_user_rt_test() {
    let client = common::setup();
    let mut response = client.get("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"status\":\"Success\",\"message\":\"Info for user 1\"}".into())
    );
}

#[test]
fn update_user_rt_test() {
    let client = common::setup();
    let mut response = client.put("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"status\":\"Success\",\"message\":\"Update info for user 1\"}".into())
    );
}

#[test]
fn delete_user_rt_test() {
    let client = common::setup();
    let mut response = client.delete("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some("{\"status\":\"Success\",\"message\":\"Delete user 1\"}".into())
    );
}
