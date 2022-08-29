//! Oversimplified http request/response abstraction layer for `bot_any` packages.
pub use client::{ClientRequest, ClientResponse};
pub use http_method::HttpMethod;
pub use http_status_code::{HttpStatusCode, HttpStatusCodeParseError};
pub use server::{ServerRequest, ServerResponse, ServerResponseBuilder, ServerResponsePart};

mod client;
mod http_method;
mod http_status_code;
mod server;
