use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;

use crate::AppState;

pub async fn get(
    State(AppState {
        engine,
        container: _,
    }): State<AppState>,
) -> impl IntoResponse {
    RenderHtml("rules", engine, ())
}
