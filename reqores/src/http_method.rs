/// The enum representing http request method
///
/// The variants' documentations are taken from [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods),
/// which is written by [Mozilla Contributors](https://developer.mozilla.org/en-US/docs/MDN/About/contributors.txt),
/// licensed under [CC=BY-SA v2.5](https://creativecommons.org/licenses/by-sa/2.5/)
pub enum HttpMethod {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    Get,

    /// The POST method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    Post,

    /// The PUT method replaces all current representations of the target resource with the request payload.
    Put,

    /// The DELETE method deletes the specified resource.
    Delete,

    /// The PATCH method applies partial modifications to a resource.
    Patch,
}
