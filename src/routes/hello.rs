use rocket::*;

#[get("/hello/<name>/<age>")]
pub fn hello_fn(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}