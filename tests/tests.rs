use rocket::http::{ContentType, Status};

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
fn user_list_rt_test(){
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"List of users\"}".into()));
}