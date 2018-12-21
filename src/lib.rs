//! # Alexa SDK
//! 
//! `alexa_sdk` implements stucts corresponding to the [Alexa JSON specification](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html)
//! along with helper functions for common uses of the  `Request` and `Response` objects.
//! 
//! ## Usage
//! 
//! Simplest possible Alexa "Hello, World" skill:
//!
//! ```rust
//! extern crate lambda_runtime as lambda;
//! extern crate alexa_sdk;
//!
//! use lambda::{lambda, Context, error::HandlerError};
//! use alexa_sdk::{Request,Response};
//! use std::error::Error;
//!
//! fn my_handler(_req: Request, _ctx: Context) -> Result<Response,HandlerError> {
//!     Ok(Response::simple("hello", "hello world"))
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     lambda!(my_handler);
//!
//!     Ok(())
//! }
//! ```
//!
//! A more complete skill, handling multiple locales and a slot:
//!
//! ```rust
//! extern crate lambda_runtime as lambda;
//! extern crate alexa_sdk;
//!
//! use lambda::{lambda, Context, error::HandlerError};
//! use alexa_sdk::{Request,Response};
//! use alexa_sdk::request::{IntentType, Locale};
//! use std::error::Error;
//!
//! fn handle_help(_req: &Request) -> Result<Response,HandlerError> {
//!     Ok(Response::simple("hello", "to say hello, tell me: say hello to someone"))
//! }
//!
//! fn handle_hello(req: &Request) -> Result<Response,HandlerError> {
//!     let res = match req.locale() {
//!         Locale::AustralianEnglish => Response::simple("hello", "G'day mate"),
//!         Locale::German => Response::simple("hello", "Hallo Welt"),
//!         Locale::Japanese => Response::simple("hello", "こんにちは世界"),
//!         _ => if let Some(ref s) = req.slot_value("name") {
//!             Response::simple("hello", (String::from("hello ") + s).as_str())
//!         } else {
//!             Response::simple("hello", "hello world")
//!         },
//!     };
//!     Ok(res)
//! }
//!
//! fn handle_cancel(_req: &Request) -> Result<Response,HandlerError> {
//!     Ok(Response::end())
//! }
//!
//! fn my_handler(req: Request, _ctx: Context) -> Result<Response,HandlerError> {
//!     match req.intent() {
//!         IntentType::Help => handle_help(&req),
//!         IntentType::User(_) => handle_hello(&req),
//!         _ => handle_cancel (&req)
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     lambda!(my_handler);
//!
//!     Ok(())
//! }
//! ```

pub mod request;
pub mod response;

pub use self::request::{Request};
pub use self::response::{Response};