use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Serializer};
use tracing::instrument;

use crate::context::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    #[serde(serialize_with = "serialize_status")]
    code: StatusCode,
    msg: String,
}

// custom serializer, because `StatusCode` does not derive Serialize
fn serialize_status<S: Serializer>(value: &StatusCode, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u16(value.as_u16())
}

impl ErrorMessage {
    pub fn new(code: StatusCode, msg: impl ToString) -> Self {
        ErrorMessage {
            code,
            msg: msg.to_string(),
        }
    }
}

impl IntoResponse for ErrorMessage {
    fn into_response(self) -> axum::response::Response {
        let code = self.code;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = code;
        res
    }
}

#[instrument(skip_all)]
pub async fn handle_404(State(_ctx): State<AppState>) -> impl IntoResponse {
    ErrorMessage::new(StatusCode::NOT_FOUND, "Not found")
}

#[instrument]
pub async fn handle_error(err: std::io::Error) -> impl IntoResponse {
    ErrorMessage::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", err),
    )
}
