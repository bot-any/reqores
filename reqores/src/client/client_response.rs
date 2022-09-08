use crate::HttpStatusCode;

/// The response made from [`ClientRequest`](`crate::ClientRequest`).
pub trait ClientResponse {
    /// The body of the response.
    fn body(&self) -> &[u8];

    /// The HTTP Status Code made from the response.
    fn status(&self) -> HttpStatusCode;

    /// The header value from the given key.
    fn header(&self, key: &str) -> Option<String>;
}
