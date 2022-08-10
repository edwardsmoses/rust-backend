#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

#[get("/")]
fn index() -> String {
    format!("Welcome world!!!")
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite()
    .attach(SpaceHelmet::default())
    .mount("/", routes![hello, index])
    .mount("/files", StaticFiles::from("static/"))
    .launch();
}