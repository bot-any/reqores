use serde::de::DeserializeOwned;

use crate::HttpMethod;

use super::ClientResponse;

pub mod headers {
    pub fn content_type_json_utf8() -> (String, String) {
        (
            "Content-Type".to_string(),
            "application/json; charset=UTF-8".to_string(),
        )
    }
}

pub trait ClientRequest {
    type Response: DeserializeOwned;

    fn headers(&self) -> Vec<(String, String)> {
        vec![headers::content_type_json_utf8()]
    }

    fn url(&self) -> String;

    fn body(&self) -> Option<String> {
        None
    }

    fn method(&self) -> HttpMethod;

    fn deserialize(&self, response: &dyn ClientResponse) -> Result<Self::Response, String> {
        serde_json::from_slice(response.body()).map_err(|e| e.to_string())
    }
}
