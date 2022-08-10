use rocket::*;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json};

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}

impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}

#[get("/users")]
pub fn user_list_rt() -> Json<Response> {
    Json(Response::ok("List of users"))
}

#[post("/users")]
pub fn new_user_rt() -> String {
    "Creation of new user".to_string()
}

#[get("/users/<id>")]
pub fn info_user_rt(id: String) -> String {
    format!("Info for user {}", id)
}

#[put("/users/<id>")]
pub fn update_user_rt(id: String) -> String {
    format!("Update info for user {}", id)
}

#[delete("/users/<id>")]
pub fn delete_user_rt(id: String) -> String {
    format!("Delete user {}", id)
}
