mod v1;

use axum::{http::Method, response::Redirect, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use v1::create_v1_routes;

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let v1 = create_v1_routes();

    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/adaptemoji/adaptemoji-api") }),
        )
        .nest("/", v1.to_owned())
        .nest("/v1", v1)
        .layer(cors)
}
