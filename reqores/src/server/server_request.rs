use serde::Deserialize;

/// The request accepted by the server.
pub trait ServerRequest {
    /// The body of the request.
    fn body(&self) -> &[u8];

    /// the body deserialized with [`serde_json::from_slice`].
    fn body_json<'a, T: Deserialize<'a>>(&'a self) -> serde_json::Result<T> {
        serde_json::from_slice(self.body())
    }

    /// The header value from the given key.
    fn header(&self, key: &str) -> Option<String>;
}
