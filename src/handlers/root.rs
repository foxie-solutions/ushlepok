use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;
use serde_json::json;

use crate::AppState;

pub async fn get(State(AppState { engine, container }): State<AppState>) -> impl IntoResponse {
    RenderHtml(
        "index",
        engine,
        json!({
            "ushlepok": container.get().map(|ushlepok| ushlepok.meta),
        }),
    )
}
