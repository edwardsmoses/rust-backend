#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;
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
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![hello, index])
        .mount("/files", StaticFiles::from("static/"))
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