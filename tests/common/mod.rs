use lazy_static::lazy_static;

use rocket::local::Client;
use rust_graphql_backend::rocket_builder;

pub fn setup() -> &'static Client {
    lazy_static! {
        static ref CLIENT: Client = Client::new(rocket_builder()).expect("Valid rocket instance");
    }
    &*CLIENT
}
