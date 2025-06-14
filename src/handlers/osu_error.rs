use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rosu_v2::error::OsuError;

pub struct OsuErrorResponse {
    status: StatusCode,
    message: String,
}

impl IntoResponse for OsuErrorResponse {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

impl From<OsuError> for OsuErrorResponse {
    fn from(error: OsuError) -> Self {
        let status = match error {
            OsuError::ParsingValue { .. } => StatusCode::BAD_REQUEST,
            OsuError::NotFound => StatusCode::NOT_FOUND,
            OsuError::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            OsuError::BuilderMissingId
            | OsuError::BuilderMissingSecret
            | OsuError::NoToken
            | OsuError::BodyError { .. }
            | OsuError::ConnectorRoots { .. }
            | OsuError::CreatingTokenHeader { .. }
            | OsuError::UpdateToken { .. }
            | OsuError::Url { .. }
            | OsuError::Parsing { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            OsuError::ChunkingResponse { .. } | OsuError::UnavailableEndpoint => {
                StatusCode::BAD_GATEWAY
            }
            OsuError::Response { status, .. } => status.to_owned(),
            OsuError::Request { .. } | OsuError::ServiceUnavailable { .. } => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = error.to_string();

        Self { status, message }
    }
}
