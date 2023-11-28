use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/static"]
struct Assets;

pub async fn get(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime.to_string())],
                content.data.to_vec(),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
