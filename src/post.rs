//! This module contains the POST routes for the API.
//!
//! The routes are:
//! - `/` - The general channel.
//! - `/<channel>` - Any specified channel.
//!
//! The routes are all asynchronous and return a `http::Status`.
//!
//! The routes are used to store messages in files.
//! The files are stored in the `./` directory.
//! The files are named after the channel they are for.
//! For example, the general channel is stored in `./general.json`.
use crate::{helpers, message::Message};
use rocket::{http, serde::json::Json};

/// This route is for the general channel.
///
/// The general channel is the default channel.
/// This route is the same as `/general`.
#[post("/", data = "<message>")]
pub async fn general(message: Json<Message>) -> http::Status {
    helpers::store_message("general", message.into_inner())
}

/// This route is for any specified channel.
///
/// The channel is specified in the URL.
/// If a channel does not exist, the message is stored in a new file.
#[post("/<channel>", data = "<message>")]
pub async fn channel(channel: String, message: Json<Message>) -> http::Status {
    helpers::store_message(&channel, message.into_inner())
}
