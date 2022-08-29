use reqores::{ClientResponse, HttpStatusCode};
use surf::{Response, StatusCode as SurfStatus};

pub struct SurfClientResponse {
    body: Vec<u8>,
    response: Response,
}

impl SurfClientResponse {
    pub async fn new(mut response: Response) -> surf::Result<Self> {
        Ok(Self {
            body: response.body_bytes().await?,
            response,
        })
    }
}

impl ClientResponse for SurfClientResponse {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn status(&self) -> HttpStatusCode {
        match self.response.status() {
            SurfStatus::Continue => HttpStatusCode::Continue,
            SurfStatus::SwitchingProtocols => HttpStatusCode::SwitchingProtocols,
            SurfStatus::EarlyHints => HttpStatusCode::EarlyHints,
            SurfStatus::Ok => HttpStatusCode::Ok,
            SurfStatus::Created => HttpStatusCode::Created,
            SurfStatus::Accepted => HttpStatusCode::Accepted,
            SurfStatus::NonAuthoritativeInformation => HttpStatusCode::NonAuthoritativeInformation,
            SurfStatus::NoContent => HttpStatusCode::NoContent,
            SurfStatus::ResetContent => HttpStatusCode::ResetContent,
            SurfStatus::PartialContent => HttpStatusCode::PartialContent,
            SurfStatus::MultiStatus => HttpStatusCode::MultiStatus,
            SurfStatus::ImUsed => HttpStatusCode::ImUsed,
            SurfStatus::MultipleChoice => HttpStatusCode::MultipleChoices,
            SurfStatus::MovedPermanently => HttpStatusCode::MovedPermanently,
            SurfStatus::Found => HttpStatusCode::Found,
            SurfStatus::SeeOther => HttpStatusCode::SeeOther,
            SurfStatus::NotModified => HttpStatusCode::NotModified,
            SurfStatus::TemporaryRedirect => HttpStatusCode::TemporaryRedirect,
            SurfStatus::PermanentRedirect => HttpStatusCode::PermanentRedirect,
            SurfStatus::BadRequest => HttpStatusCode::BadRequest,
            SurfStatus::Unauthorized => HttpStatusCode::Unauthorized,
            SurfStatus::PaymentRequired => HttpStatusCode::PaymentRequired,
            SurfStatus::Forbidden => HttpStatusCode::Forbidden,
            SurfStatus::NotFound => HttpStatusCode::Notfound,
            SurfStatus::MethodNotAllowed => HttpStatusCode::MethodNotAllowed,
            SurfStatus::NotAcceptable => HttpStatusCode::NotAcceptable,
            SurfStatus::ProxyAuthenticationRequired => HttpStatusCode::ProxyAuthenticationRequired,
            SurfStatus::RequestTimeout => HttpStatusCode::RequestTimeout,
            SurfStatus::Conflict => HttpStatusCode::Conflict,
            SurfStatus::Gone => HttpStatusCode::Gone,
            SurfStatus::LengthRequired => HttpStatusCode::LengthRequired,
            SurfStatus::PreconditionFailed => HttpStatusCode::PreconditionFailed,
            SurfStatus::PayloadTooLarge => HttpStatusCode::PayloadTooLarge,
            SurfStatus::UriTooLong => HttpStatusCode::UriTooLong,
            SurfStatus::UnsupportedMediaType => HttpStatusCode::UnsupportedMediaType,
            SurfStatus::RequestedRangeNotSatisfiable => HttpStatusCode::RangeNotSatisfiable,
            SurfStatus::ExpectationFailed => HttpStatusCode::ExpectationFailed,
            SurfStatus::ImATeapot => HttpStatusCode::ImATeapot,
            SurfStatus::MisdirectedRequest => HttpStatusCode::MisdirectedRequest,
            SurfStatus::UnprocessableEntity => HttpStatusCode::UnprocessableEntity,
            SurfStatus::Locked => HttpStatusCode::Locked,
            SurfStatus::FailedDependency => HttpStatusCode::FailedDependency,
            SurfStatus::TooEarly => HttpStatusCode::TooEarly,
            SurfStatus::UpgradeRequired => HttpStatusCode::UpgradeRequired,
            SurfStatus::PreconditionRequired => HttpStatusCode::PreconditionRequired,
            SurfStatus::TooManyRequests => HttpStatusCode::TooManyRequests,
            SurfStatus::RequestHeaderFieldsTooLarge => HttpStatusCode::RequestHeaderFieldsTooLarge,
            SurfStatus::UnavailableForLegalReasons => HttpStatusCode::UnavailableForLegalReasons,
            SurfStatus::InternalServerError => HttpStatusCode::InternalServerError,
            SurfStatus::NotImplemented => HttpStatusCode::NotImplemented,
            SurfStatus::BadGateway => HttpStatusCode::BadGateway,
            SurfStatus::ServiceUnavailable => HttpStatusCode::ServiceUnavailable,
            SurfStatus::GatewayTimeout => HttpStatusCode::GatewayTimeout,
            SurfStatus::HttpVersionNotSupported => HttpStatusCode::HttpVersionNotSupported,
            SurfStatus::VariantAlsoNegotiates => HttpStatusCode::VariantAlsoNegotiates,
            SurfStatus::InsufficientStorage => HttpStatusCode::InsufficientStorage,
            SurfStatus::LoopDetected => HttpStatusCode::LoopDetected,
            SurfStatus::NotExtended => HttpStatusCode::NotExtended,
            SurfStatus::NetworkAuthenticationRequired => {
                HttpStatusCode::NetworkAuthenticationRequired
            }
        }
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.header(key).map(|v| v.to_string())
    }
}
