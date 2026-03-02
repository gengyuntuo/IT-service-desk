use axum::Router;
use axum::routing::get;
use tracing::{Level, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod dto;
mod middleware;
mod routes;

// 1. 收集所有 OpenAPI 文档
// 我们需要手动列出所有包含 #[utoipa::path] 的函数所在的模块或直接引用 ApiDoc
// 更好的做法是在每个 router 文件里定义局部的 OpenApi，或者在这里统一收集
#[derive(OpenApi)]
#[openapi(
    paths(),
    components(
        schemas(
            // dto::common::ApiResponse
        )
    ),
    tags(
        (name = "User", description = "用户管理接口"),
        (name = "System", description = "系统监控接口")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/health-check", get(|| async { "OK" }))
        // 挂载 Swagger UI
        .merge(SwaggerUi::new("/swagger").url("/swagger/openapi.json", ApiDoc::openapi()))
        .into_make_service();

    info!("Starting server on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
