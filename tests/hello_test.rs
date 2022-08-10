use rocket::http::Status;
use rocket::local::Client;
use rust_graphql_backend::rocket_builder;

#[test]
fn hello_test() {
    let client = Client::new(rocket_builder()).expect("Valid rocket instance");
    let mut response = client.get("/hello/edwards/23").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("Hello, 23 year old named edwards!".into())
    );
}
