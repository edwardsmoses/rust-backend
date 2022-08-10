use rocket::local::Client;
use rust_graphql_backend::rocket_builder;


pub fn setup() -> Client {
  Client::new(rocket_builder()).expect("Valid rocket instance")
}