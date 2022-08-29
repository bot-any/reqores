use reqores::ServerResponse;
use worker::Response;

pub fn encode_response(server_response: ServerResponse) -> worker::Result<Response> {
    let mut response = Response::from_bytes(server_response.body.unwrap_or_default())?;
    if let Some(code) = server_response.status {
        response = response.with_status(u16::from(code));
    }

    for (name, value) in response.headers() {
        response.headers_mut().set(&name, &value)?;
    }

    Ok(response)
}
