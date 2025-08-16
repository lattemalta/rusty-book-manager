use axum::{http::StatusCode, response::IntoResponse};
use sqlx;
use thiserror::Error;
use tracing;
use uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("データベース処理中にエラーが発生しました")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
            AppError::ConvertToUuidError(_) => StatusCode::BAD_REQUEST,
            e @ AppError::SpecificOperationError(_) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
