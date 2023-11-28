use std::{io::Cursor, time::SystemTime};

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_template::RenderHtml;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use garde::Validate;

use image::{io::Reader as ImageReader, EncodableLayout};
use serde_json::json;

use crate::{
    container::{Container, Ushlepok, UshlepokMeta},
    AppState,
};

pub async fn get(State(AppState { engine, .. }): State<AppState>) -> impl IntoResponse {
    RenderHtml("add", engine, ())
}

#[derive(TryFromMultipart, Validate)]
pub struct AddUshlepok {
    #[garde(skip)]
    #[form_data(limit = "unlimited")]
    image: axum::body::Bytes,
    #[garde(length(min = 1, max = 100))]
    description: String,
    #[garde(url)]
    redirect_url: String,
    #[garde(required)]
    rules: Option<String>,
}

pub async fn post(
    State(AppState { container, engine }): State<AppState>,
    data: TypedMultipart<AddUshlepok>,
) -> impl IntoResponse {
    match post_internal(container, data).await {
        Ok(_) => Redirect::to("/").into_response(),
        Err(e) => RenderHtml(
            "add",
            engine,
            json!({
                "error": e.to_string()
            }),
        )
        .into_response(),
    }
}

async fn post_internal(
    container: Container,
    data: TypedMultipart<AddUshlepok>,
) -> Result<(), Box<dyn std::error::Error>> {
    data.validate(&())?;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if container
        .get()
        .map(|u| now < u.meta.timestamp + 60 * 10)
        .unwrap_or(false)
    {
        return Err(garde::Error::new("you can't add ushlepok so often pls wait").into());
    }

    let img = ImageReader::new(Cursor::new(data.image.as_bytes()))
        .with_guessed_format()
        .map_err(|_| garde::Error::new("invalid image format"))
        .and_then(|decoder| {
            decoder
                .decode()
                .map_err(|_| garde::Error::new("invalid image"))
        })?;

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(100))?;

    container.set(Ushlepok {
        meta: UshlepokMeta {
            description: data.description.clone(),
            redirect_url: data.redirect_url.clone(),
            timestamp: now,
        },
        image: buf.into_inner(),
    })?;

    Ok(())
}
