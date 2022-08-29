use crate::HttpStatusCode;

pub trait ClientResponse {
    fn body(&self) -> &[u8];

    fn status(&self) -> HttpStatusCode;

    fn header(&self, key: &str) -> Option<String>;
}
