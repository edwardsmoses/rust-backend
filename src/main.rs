#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] use rocket::*;
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> String {
    format!("Welcome world!!!")
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/file/<file..>")]
fn fileserver(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


fn main() {
    rocket::ignite().mount("/", routes![hello, index, fileserver]).launch();
}