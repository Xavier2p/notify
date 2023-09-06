use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    title: String,
    message: String,
    style: String,
}

impl Message {
    pub fn new(title: String, message: String, style: String) -> Json<Message> {
        Json(Message {
            title,
            message,
            style: format!("is-{}", style),
        })
    }
}
