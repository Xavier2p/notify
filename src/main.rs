//! > *A simple API to send messages to your Dashboard*
//!
//! ## Routes
//!
//! + `GET` - `/` - Get the current general message
//! + `GET` - `/<channel>` - Get the current message in the channel
//! + `GET` - `/health` - Get the health of the service
//! + `POST` - `/` - Send a message to the general channel
//! + `POST` - `/<channel>` - Send a message to a specific channel
//!
//! ## Message Format
//!
//! ```jsonc
//! {
//!   "message": "Hello World!",
//!   "title": "Hello",
//!   "style": "info" // can be `info`, `success`, `warning` or `danger`
//! }
//! ```
//!
//! ## Deployment
//!
//! ### Local build
//!
//! ```bash
//! # Clone repo
//! git clone https://github.com/Xavier2p/notify.git && cd notify
//!
//! # Build the image
//! docker build -t notifier .
//!
//! # Run the image
//! docker run -d --rm -p 8080:8000 --name notifier notifier
//! ```
//!
//! ### From GHCR
//!
//! ```bash
//! docker run -d --rm -p 8080:8000 --name notifier ghcr.io/xavier2p/notify:latest
//! ```
//!
//! ## License - MIT
//!
//! > Copyright (c) 2023 Xavier2p
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
mod get;
mod helpers;
mod message;
mod post;

#[macro_use]
extern crate rocket;

/// This function starts the application and mounts the routes.
///
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
