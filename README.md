# Rust Request/Response for Amazon Alexa Skills

## About

Implements Amazon Alexa Skill request/response structs following the [Alexa skill specifications](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html), including Serde JSON serialization/deserialization with some helpers to extract data from requests and format responses.

These structs can be used as the request and response using the Rust AWS Lambda runtime to implement the skill.

## Usage

Simplest possible Alexa "Hello, World" skill:

```rust
extern crate lambda_runtime as lambda;
extern crate alexa;

use lambda::{lambda, Context, error::HandlerError};
use alexa::{Request,Response};
use std::error::Error;


fn my_handler(_req: Request, _ctx: Context) -> Result<Response,HandlerError> {
    Ok(Response::new_simple("hello", "hello world"))
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);

    Ok(())
}
```
