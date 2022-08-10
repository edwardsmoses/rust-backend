#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> String {
    format!("Welcome world!!!")
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, index]).mount("/files", StaticFiles::from("static/")).launch();
}