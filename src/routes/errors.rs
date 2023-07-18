use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::instrument;

use crate::context::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    code: u32,
    msg: String,
}

#[instrument(skip(ctx))]
pub async fn handle_404(State(ctx): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::Json(ErrorMessage {
            code: 404,
            msg: "Not found".to_string(),
        }),
    )
}

#[instrument]
pub async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::Json(ErrorMessage {
            code: 404,
            msg: "Not found".to_string(),
        }),
    )
}
