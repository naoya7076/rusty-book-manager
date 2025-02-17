use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use registry::AppRegistry;
use thiserror::Error;

use crate::model::book::CreateBookRequest;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}
