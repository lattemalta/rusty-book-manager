use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> AppResult<Json<Vec<BookResponse>>> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|books| {
            books
                .into_iter()
                .map(BookResponse::from)
                .collect::<Vec<_>>()
        })
        .map(Json)
}

pub async fn show_book(
    Path(book_id): Path<Uuid>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(book) => Ok(Json(book.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".to_string(),
            )),
        })
}
