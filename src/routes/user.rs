use rocket::*;


#[get("/users")]
pub fn user_list_rt() -> String {
    "List of Users".to_string()
}

#[post("/users")]
pub fn new_user_rt() -> String{
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