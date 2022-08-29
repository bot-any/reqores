use serde::de::DeserializeOwned;

use crate::HttpMethod;

use super::ClientResponse;

pub trait ClientRequest {
    type Response: DeserializeOwned;

    fn headers(&self) -> Vec<(String, String)> {
        Default::default()
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
