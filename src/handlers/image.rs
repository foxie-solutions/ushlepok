use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use mime_guess::mime::IMAGE_JPEG;

use crate::{AppState, IMAGE_PATH};

pub async fn get(State(AppState { container, .. }): State<AppState>) -> impl IntoResponse {
    match (container.get(), std::fs::read(IMAGE_PATH)) {
        (Some(_), Ok(image)) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, IMAGE_JPEG.to_string())],
            image,
        )
            .into_response(),
        _ => (StatusCode::NO_CONTENT, "нет ушлепка =(").into_response(),
    }
}
