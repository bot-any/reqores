//! # reqores
//! 
//! Oversimplified http request/response abstraction layer for `bot_any` packages.
//! 
//! - Simple, lightweight abstraction
//! - Easily exchangeable client/server implementation
//! 
//! # Implementations
//! 
//! - [reqores-client-surf](https://crates.io/crates/reqores-client-surf) - surf based, async client implementation
//! - [reqores-universal-cf-worker](https://crates.io/crates/reqores-universal-cf-worker) - universal client/server implementation for Cloudflare Workers
//! 
//! # Example
//! 
//! Define a new [`ClientRequest`] for your web API.
//! 
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! # use reqores::{ClientRequest, HttpMethod};
//! #
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct YourApiResponse { /* ... some fields ... */ }
//! 
//! pub struct YourApiCall;
//! 
//! impl ClientRequest for YourApiCall {
//!     type Response = YourApiResponse;
//! 
//!     fn url(&self) -> String {
//!         "https://example.com/api".to_string()
//!     }
//! 
//!     fn method(&self) -> HttpMethod {
//!         HttpMethod::Get
//!     }
//! }
//! ```

#![deny(missing_docs)]

pub use client::{headers, ClientRequest, ClientResponse};
pub use http_method::HttpMethod;
pub use http_status_code::{HttpStatusCode, HttpStatusCodeParseError};
pub use server::{ServerRequest, ServerResponse, ServerResponseBuilder};

mod client;
mod http_method;
mod http_status_code;
mod server;
