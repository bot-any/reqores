use serde::Serialize;

use crate::HttpStatusCode;

pub enum ServerResponsePart {
    Header(String, String),
    StatusCode(HttpStatusCode),
}

pub struct ServerResponse {
    pub parts: Vec<ServerResponsePart>,
    pub body: Option<Vec<u8>>,
}

impl ServerResponse {
    pub fn then(self, other: ServerResponse) -> ServerResponse {
        if self.body.is_some() {
            self
        } else {
            let mut parts = self.parts;
            parts.extend(other.parts);
            ServerResponse {
                parts,
                body: other.body,
            }
        }
    }
}

pub struct ServerResponseBuilder {
    parts: Vec<ServerResponsePart>,
    body: Option<Vec<u8>>,
}

impl ServerResponseBuilder {
    pub fn new() -> Self {
        ServerResponseBuilder {
            body: None,
            parts: Vec::new(),
        }
    }

    pub fn header(mut self, name: String, value: String) -> Self {
        self.parts.push(ServerResponsePart::Header(name, value));
        self
    }

    pub fn status(mut self, status: HttpStatusCode) -> Self {
        self.parts.push(ServerResponsePart::StatusCode(status));
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> ServerResponse {
        self.body = Some(body);
        self.build()
    }

    pub fn body_str(self, body: &str) -> ServerResponse {
        self.body(body.as_bytes().to_vec())
    }

    pub fn body_json<T: Serialize>(self, body: &T) -> serde_json::Result<ServerResponse> {
        Ok(self
            .header(
                "Content-Type".to_string(),
                "application/json; charset=UTF-8".to_string(),
            )
            .body(serde_json::to_vec(body)?))
    }

    pub fn build(self) -> ServerResponse {
        ServerResponse {
            parts: self.parts,
            body: self.body,
        }
    }
}
