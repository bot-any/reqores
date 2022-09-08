use serde::Serialize;

use crate::HttpStatusCode;

/// The response made from the server.
/// 
/// Note that the response could be partial and composable. 
pub struct ServerResponse {
    /// The status code of the response.
    pub status: Option<HttpStatusCode>,
    /// The headers of the response.
    pub headers: Vec<(String, String)>,
    /// The body of the response.
    pub body: Option<Vec<u8>>,
}

impl ServerResponse {
    /// Compose with next response.
    /// If self is having the body, the next response will be ignored.
    pub fn then(self, other: ServerResponse) -> ServerResponse {
        if self.body.is_some() {
            self
        } else {
            let mut headers = self.headers;
            headers.extend(other.headers);
            ServerResponse {
                status: other.status.or(self.status),
                headers,
                body: other.body,
            }
        }
    }
}

/// The utility builder for [`ServerResponse`].
#[derive(Default)]
pub struct ServerResponseBuilder {
    status: Option<HttpStatusCode>,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl ServerResponseBuilder {
    /// Create a new [`ServerResponseBuilder`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Attach header to the builder.
    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.push((name, value));
        self
    }

    /// Set status to the builder.
    pub fn with_status(mut self, status: HttpStatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Set body to the builder and build [`ServerResponse`].
    pub fn body(mut self, body: Vec<u8>) -> ServerResponse {
        self.body = Some(body);
        self.end()
    }

    /// Set body with string content to the builder and build [`ServerResponse`].
    pub fn body_str(self, body: &str) -> ServerResponse {
        self.body(body.as_bytes().to_vec())
    }

    /// Set body with serializable json content to the builder and build [`ServerResponse`].
    pub fn body_json<T: Serialize>(self, body: &T) -> serde_json::Result<ServerResponse> {
        Ok(self
            .with_header(
                "Content-Type".to_string(),
                "application/json; charset=UTF-8".to_string(),
            )
            .body(serde_json::to_vec(body)?))
    }

    /// Build [`ServerResponse`] without body.
    pub fn end(self) -> ServerResponse {
        ServerResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}
