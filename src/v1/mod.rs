mod convert;

use axum::{routing::post, Router};
use convert::convert;
use convert_raw::convert_raw;

pub fn create_v1_routes() -> Router {
    Router::new().route("/convert", post(convert))
}
