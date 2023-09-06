use crate::message::Message;
use rocket::{http, serde::json::Json};
use std::fs::File;

pub fn retrieve_message(filename: &str) -> Result<Json<Message>, http::Status> {
    let path: String = format!("{}.json", filename);
    match File::open(path) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(message) => Ok(Json(message)),
            Err(_) => Err(http::Status::InternalServerError),
        },
        Err(_) => Err(http::Status::NotFound),
    }
}

pub fn store_message(filename: &str, message: Message) -> http::Status {
    let path: String = format!("{}.json", filename);
    match File::create(path) {
        Ok(file) => match serde_json::to_writer(file, &message) {
            Ok(_) => http::Status::Ok,
            Err(_) => http::Status::InternalServerError,
        },
        Err(_) => http::Status::InternalServerError,
    }
}
