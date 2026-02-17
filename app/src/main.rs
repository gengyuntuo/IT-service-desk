use axum::Router;
use axum::routing::get;
use tracing::{Level, info};
use utoipa_swagger_ui::SwaggerUi;

mod dto;
mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let swagger_ui = SwaggerUi::new("/swagger");
    let app = Router::new()
        .route("/health-check", get(|| async { "OK" }))
        .merge(swagger_ui)
        .into_make_service();

    info!("Starting server on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
