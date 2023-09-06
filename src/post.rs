//! This module contains the POST routes for the API.
use crate::{helpers, message::Message};
use rocket::{http, serde::json::Json};

/// This route is for the general channel.
#[post("/", data = "<message>")]
pub async fn general(message: Json<Message>) -> http::Status {
    helpers::store_message("general", message.into_inner())
}

/// This route is for any specified channel.
#[post("/<channel>", data = "<message>")]
pub async fn channel(channel: String, message: Json<Message>) -> http::Status {
    helpers::store_message(&channel, message.into_inner())
}
