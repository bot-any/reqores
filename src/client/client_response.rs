use serde::Deserialize;

use crate::HttpStatusCode;

pub trait ClientResponse {
    fn body(&self) -> &[u8];

    fn body_json<'a, T: Deserialize<'a>>(&'a self) -> serde_json::Result<T> {
        serde_json::from_slice(self.body())
    }

    fn status(&self) -> HttpStatusCode;

    fn header(&self, key: &str) -> Option<String>;
}
