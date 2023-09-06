//! # Main
//! The main file for the application.
//! This file is responsible for starting the application and mounting the routes.
mod get;
mod helpers;
mod message;
mod post;

#[macro_use]
extern crate rocket;

/// This function starts the application and mounts the routes.
/// The routes are mounted in the order they are listed.
/// The first route is the most specific route, and the last route is the least specific route.
/// The routes are mounted in this order to prevent the health check from being overridden by the other routes.
/// The health check is the least specific route, so it is mounted last.
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get::channel,
            get::general,
            get::health,
            post::channel,
            post::general
        ],
    )
}
