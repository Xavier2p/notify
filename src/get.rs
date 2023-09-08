//! This module contains the GET routes for the API.
//!
//! The routes are:
//! - `/` - The general channel.
//! - `/<channel>` - Any specified channel.
//! - `/health` - The health check.
//!
//! The routes are all asynchronous and return a `Result<Json<Message>, http::Status>`.
use crate::{helpers, message::Message};
use rocket::{http, serde::json::Json};

/// This route is for the general channel.
///
/// The general channel is the default channel.
/// This route is the same as `/general`.
#[get("/")]
pub async fn general() -> Result<Json<Message>, http::Status> {
    helpers::retrieve_message("general")
}

/// This route is for any specified channel.
///
/// The channel is specified in the URL.
/// If a channel does not exist, a `404` error is returned.
#[get("/<channel>")]
pub async fn channel(channel: String) -> Result<Json<Message>, http::Status> {
    helpers::retrieve_message(&channel)
}

/// This route is for the health check.
///
/// The health check is used to check if the API is up and running.
/// It returns a `200` status code if everything is good.
#[get("/health")]
pub async fn health() -> Json<Message> {
    Message::new(
        "Health".to_string(),
        "Everything is good!".to_string(),
        "success".to_string(),
    )
}
