use adaptemoji::AdaptiveEmojiConvert;
use anyhow::Result;
use axum::{
    body::{Body, Bytes},
    extract::{Multipart, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use image::{io::Reader as ImageReader, GrayAlphaImage, ImageFormat};
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize, Debug, Clone)]
pub struct ConvertQuery {
    negative: bool,
    resize: bool,
}

pub async fn convert(query: Query<ConvertQuery>, mut multipart: Multipart) -> Response {
    let option_field = match multipart.next_field().await {
        Ok(option_field) => option_field,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e)).into_response();
        }
    };

    let field = match option_field {
        Some(field) => field,
        None => {
            return (StatusCode::BAD_REQUEST, "Empty multipart".to_string()).into_response();
        }
    };

    let bytes = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Reading image error: {}", e),
            )
                .into_response()
        }
    };

    let img = match convert_image(&bytes, &query) {
        Ok(img) => img,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Image conversion error: {}", e),
            )
                .into_response()
        }
    };

    // TODO: Check if it is the best way to do it
    let mut cursor = Cursor::new(Vec::new());

    if let Err(e) = img.write_to(&mut cursor, ImageFormat::Png) {
        return (
            StatusCode::BAD_REQUEST,
            format!("Image conversion error: {}", e),
        )
            .into_response();
    }

    let body = Body::from(cursor.into_inner());
    (StatusCode::OK, body).into_response()
}

pub fn convert_image(bytes: &Bytes, query: &Query<ConvertQuery>) -> Result<GrayAlphaImage> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    let adaptive_image = if query.resize {
        img.resize(100, 100, image::imageops::FilterType::Triangle)
    } else {
        img
    }
    .to_luma_alpha8()
    .convert_adaptive(query.negative);

    Ok(adaptive_image)
}
