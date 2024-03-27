use adaptemoji_api::create_routes;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_routes();

    let port = env::var("PORT").map_or(3000, |port| port.parse().unwrap());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
