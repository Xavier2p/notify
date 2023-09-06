//! This module contains the GET routes for the API.
use crate::{helpers, message::Message};
use rocket::{http, serde::json::Json};

/// This route is for the general channel.
#[get("/")]
pub async fn general() -> Result<Json<Message>, http::Status> {
    helpers::retrieve_message("general")
}

/// This route is for any specified channel.
#[get("/<channel>")]
pub async fn channel(channel: String) -> Result<Json<Message>, http::Status> {
    helpers::retrieve_message(&channel)
}

/// This route is for the health check.
#[get("/health")]
pub async fn health() -> Json<Message> {
    Message::new(
        "Health".to_string(),
        "Everything is good!".to_string(),
        "success".to_string(),
    )
}
