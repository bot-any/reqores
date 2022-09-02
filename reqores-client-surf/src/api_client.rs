use reqores::{ClientRequest, HttpMethod};
use surf::Client;

use crate::client_response::SurfClientResponse;

pub struct SurfClient(Client);

impl SurfClient {
    pub fn new() -> Self {
        SurfClient(Client::new())
    }
    pub fn with_client(client: Client) -> Self {
        SurfClient(client)
    }
}

impl SurfClient {
    pub async fn call<Req: ClientRequest>(
        &self,
        client_request: Req,
    ) -> surf::Result<Req::Response> {
        let mut request = match client_request.method() {
            HttpMethod::Get => self.0.get(&client_request.url()),
            HttpMethod::Put => self.0.put(&client_request.url()),
            HttpMethod::Post => self.0.post(&client_request.url()),
            HttpMethod::Delete => self.0.delete(&client_request.url()),
            HttpMethod::Patch => self.0.patch(&client_request.url()),
        };
        request = request.header("Content-Type", "application/json; charset=UTF-8");
        for (k, v) in client_request.headers() {
            request = request.header(&*k, v);
        }

        if let Some(body) = client_request.body() {
            request = request.body(body);
        }

        let response = request.send().await?;
        let client_response = SurfClientResponse::new(response).await?;

        client_request
            .deserialize(&client_response)
            .map_err(|s| surf::Error::from_str(500, s))
    }
}
