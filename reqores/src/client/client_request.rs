use serde::de::DeserializeOwned;

use crate::HttpMethod;

use super::ClientResponse;

/// Collections for common headers
pub mod headers {
    /// Header entry for setting "Content-Type" with "application/json; charset=UTF-8"
    pub fn content_type_json_utf8() -> (String, String) {
        (
            "Content-Type".to_string(),
            "application/json; charset=UTF-8".to_string(),
        )
    }
}

/// A client request to send
pub trait ClientRequest {
    /// The type for response.
    type Response: DeserializeOwned;

    /// The headers to send.
    /// 
    /// By default, it will set "Content-Type" to "application/json; charset=UTF-8".
    fn headers(&self) -> Vec<(String, String)> {
        vec![headers::content_type_json_utf8()]
    }

    /// The URL endpoint.
    fn url(&self) -> String;

    /// The request body to send.
    fn body(&self) -> Option<String> {
        None
    }

    /// The HTTP method to use.
    fn method(&self) -> HttpMethod;

    /// The way to deserialize the response.
    /// 
    /// By default, it will use [`serde_json::from_slice`] for deserializing response body.
    fn deserialize(&self, response: &dyn ClientResponse) -> Result<Self::Response, String> {
        serde_json::from_slice(response.body()).map_err(|e| e.to_string())
    }
}
