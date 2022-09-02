use reqores::{ClientRequest, HttpMethod};
use worker::{wasm_bindgen::JsValue, Fetch, Headers, Method, Request, RequestInit};

use super::client_response::CfWorkerClientResponse;

pub struct CfWorkerClient;

impl CfWorkerClient {
    pub async fn call<Req: ClientRequest>(
        &self,
        client_request: Req,
    ) -> Result<Req::Response, worker::Error> {
        let mut headers = Headers::new();
        for (k, v) in client_request.headers() {
            headers.set(&k, &v)?;
        }

        let mut request_init = RequestInit::new();
        request_init
            .with_method(match client_request.method() {
                HttpMethod::Get => Method::Get,
                HttpMethod::Post => Method::Post,
                HttpMethod::Put => Method::Put,
                HttpMethod::Delete => Method::Delete,
                HttpMethod::Patch => Method::Patch,
            })
            .with_headers(headers)
            .with_body(client_request.body().map(|s| JsValue::from_str(&s)));

        let request = Fetch::Request(Request::new_with_init(
            &client_request.url(),
            &request_init,
        )?);
        let response = request.send().await?;
        let client_response = CfWorkerClientResponse::new(response).await?;

        let result = client_request
            .deserialize(&client_response)
            .map_err(worker::Error::RustError)?;

        Ok(result)
    }
}
