//! This module contains the helper functions for the API.
//!
//! The helper functions are:
//! - `retrieve_message` - Retrieves a message from a file.
//! - `store_message` - Stores a message in a file.
//!
//! The helper functions are used by the routes in `src/get.rs` and `src/post.rs`.
//! The helper functions are all synchronous and return a `Result<Json<Message>, http::Status>`.
//!
//! The helper functions are used to retrieve and store messages in files.
//! The files are stored in the `./` directory.
//! The files are named after the channel they are for.
//! For example, the general channel is stored in `./general.json`.
use crate::message::Message;
use rocket::{http, serde::json::Json};
use std::fs::File;

/// This function retrieves a message from a file.
///
/// ## Arguments:
/// - `filename` - The name of the file to retrieve the message from.
///
/// ## Returns:
/// - `Ok(Json<Message>)` - The message from the file.
/// - `Err(http::Status)` - The status code of the error.
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

/// This function stores a message in a file.
///
/// ## Arguments:
/// - `filename` - The name of the file to store the message in.
/// - `message` - The message to store in the file.
///
/// ## Returns:
/// - `http::Status` - The status code of the error, 200 if no error.
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
