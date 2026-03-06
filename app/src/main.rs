use axum::routing::get;
use axum::Router;
use respository::create_pool;
use sqlx::{Pool, Postgres};
use tracing::{info, Level};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod dto;
mod error;
mod middleware;
mod routes;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

// 1. 收集所有 OpenAPI 文档
// 在每个路由模块中定义局部的 OpenApi，然后在这里统一收集
#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/api/v1", api=crate::routes::tickets::TicketsApiDoc),
    ),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/health-check", get(|| async { "OK" }))
        // 挂载 Swagger UI
        .merge(SwaggerUi::new("/swagger").url("/swagger/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1/tickets", crate::routes::tickets::routers())
        .with_state(AppState {
            db: create_pool().await.unwrap(),
        })
        .into_make_service();

    info!("Starting server on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
