# Rust Request/Response for Amazon Alexa Skills

## About

Implements Amazon Alexa Skill request/response structs following the [Alexa skill specifications](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html), including Serde JSON serialization/deserialization with some helpers to extract data from requests and format responses.

These structs can be used as the request and response using the Rust AWS Lambda runtime to implement the skill.

## Usage

Simplest possible Alexa "Hello, World" skill:

```rust
extern crate lambda_runtime as lambda;
extern crate alexa_sdk;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use std::error::Error;


fn my_handler(_req: Request, _ctx: Context) -> Result<Response,HandlerError> {
    Ok(Response::simple("hello", "hello world"))
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);

    Ok(())
}
```

A more complete skill, handling multiple locales and a slot:

```rust
extern crate lambda_runtime as lambda;
extern crate alexa_sdk;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType, Locale};
use std::error::Error;

fn handle_help(_req: &Request) -> Result<Response,HandlerError> {
    Ok(Response::simple("hello", "to say hello, tell me: say hello to someone"))
}

fn handle_hello(req: &Request) -> Result<Response,HandlerError> {
    let res = match req.locale() {
        Locale::AustralianEnglish => Response::simple("hello", "G'day mate"),
        Locale::German => Response::simple("hello", "Hallo Welt"),
        Locale::Japanese => Response::simple("hello", "こんにちは世界"),
        _ => if let Some(ref s) = req.slot_value("name") {
            Response::simple("hello", (String::from("hello ") + s).as_str())
        } else {
            Response::simple("hello", "hello world")
        },
    };
    Ok(res)
}

fn handle_cancel(_req: &Request) -> Result<Response,HandlerError> {
    Ok(Response::end())
}

fn my_handler(req: Request, _ctx: Context) -> Result<Response,HandlerError> {
    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::Cancel => handle_cancel(&req),
        IntentType::User(_) => handle_hello(&req),
        _ => handle_cancel (&req)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);

    Ok(())
}
```

### Reponse Builder

Besides the simplifed Response constructors `Response::simple` and `Response::end` (to end a session), the SDK supports a Response builder:

```rust
let img = Image {
    small_image_url: Some(String::from("https://example.com/baaz.png")),
    large_image_url: Some(String::from("https://example.com/baazLarge.png"))
};
let mut res = Response::new(false) // should not end session
    .card(Card::standard("foo", "bar", img))
    .speech(Speech::plain("hello"));
res.add_attribute("attr", "value");
```

### Attributes

Alexa skills support attributes, which can be used to carry simple state in a session. To set an attribute in the response, use `add_attribute` on the response, to read a previously set attribute on a subsequent request, use `attribute_value` on the request.

[TODO: example]