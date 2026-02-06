use axum::routing::get;
use axum::{Json, Router};
use tracing::{Level, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod dto;
mod middleware;
mod routes;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let swagger_ui = SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi());
    let app = Router::new()
        .route("/", get(|| async { "Hello!" }))
        .merge(swagger_ui)
        .into_make_service();

    info!("Starting server on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
