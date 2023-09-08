//! This module contains the struct `Message`.
//!
//! The struct `Message` is used to store the message data.
//! It's used by the helper functions in `src/helpers.rs`, `src/get.rs` and `src/post.rs`.
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

/// This struct is used to store the message data.
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    /// The title of the message.
    title: String,

    /// The message.
    message: String,

    /// The style of the message. Can be `info`, `success`, `warning` or `danger`.
    style: String,
}

/// This is the implementation of the `Message` struct.
impl Message {
    /// This function creates a new `Message` struct.
    ///
    /// ## Arguments:
    /// - `title` - The title of the message.
    /// - `message` - The message.
    /// - `style` - The style of the message. Can be `info`, `success`, `warning` or `danger`.
    ///
    /// ## Returns:
    /// - `Json<Message>` - The new `Message` struct.
    pub fn new(title: String, message: String, style: String) -> Json<Message> {
        Json(Message {
            title,
            message,
            style: format!("is-{}", style),
        })
    }
}
