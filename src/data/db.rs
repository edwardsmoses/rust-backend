use argon2::{self, hash_encoded, Config};
use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPassword {
    pub password: String,
    pub new_password: Option<String>,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        //create random salt
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let mut config = Config::default();
        let hashed_password =
            hash_encoded(&password.as_bytes(), &salt.as_bytes(), &config).unwrap();

        User {
            id: Uuid::new_v4(),
            name,
            email,
            hashed_password,
            salt,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.name, insertable.email, insertable.password)
    }

    pub fn match_password(&self, password: &String) -> bool {
        argon2::verify_encoded(&self.hashed_password, password.as_bytes()).unwrap()
    }

    pub fn update_password(&mut self, password: &String) {
        let mut config = Config::default();

        self.hashed_password =
            hash_encoded(password.as_bytes(), self.salt.as_bytes(), &config).unwrap();
        self.updated = Utc::now();
    }

    pub fn update_user(&mut self, name: &String, email: &String) {
        self.name = name.to_string();
        self.email = email.to_string();
        self.updated = Utc::now();
    }
}

impl ResponseUser {
    pub fn from_user(user: &User) -> Self {
        ResponseUser {
            id: user.id.to_string(),
            name: format!("{}", user.name),
            email: format!("{}", user.email),
        }
    }
}
