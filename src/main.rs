use rust_graphql_backend::rocket_builder;


fn main() {
    rocket_builder().launch();
}






#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_test() {
        let client = Client::new(rocket()).expect("Valid rocket instance");
        let mut response = client.get("/hello/edwards/23").dispatch();
        assert_eq!(response.status(), Status::Ok); 
        assert_eq!(response.body_string(), Some("Hello, 23 year old named edwards!".into()));
    }
}