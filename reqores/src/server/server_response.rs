use serde::Serialize;

use crate::HttpStatusCode;

pub struct ServerResponse {
    pub status: Option<HttpStatusCode>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl ServerResponse {
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

#[derive(Default)]
pub struct ServerResponseBuilder {
    status: Option<HttpStatusCode>,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl ServerResponseBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.push((name, value));
        self
    }

    pub fn with_status(mut self, status: HttpStatusCode) -> Self {
        self.status = Some(status);
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> ServerResponse {
        self.body = Some(body);
        self.end()
    }

    pub fn body_str(self, body: &str) -> ServerResponse {
        self.body(body.as_bytes().to_vec())
    }

    pub fn body_json<T: Serialize>(self, body: &T) -> serde_json::Result<ServerResponse> {
        Ok(self
            .with_header(
                "Content-Type".to_string(),
                "application/json; charset=UTF-8".to_string(),
            )
            .body(serde_json::to_vec(body)?))
    }

    pub fn end(self) -> ServerResponse {
        ServerResponse {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}
