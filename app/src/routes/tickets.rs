use crate::dto::tickets::{CreateTicketReq, Ticket};
use axum::Router;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::routing::post;
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/tickets",
    request_body = CreateTicketReq,
    responses(
        (status = 201, description = "Ticket created successfully", body = Ticket),
    )
)]
pub async fn create_ticket(
    Json(payload): Json<CreateTicketReq>,
) -> Result<Json<Ticket>, (StatusCode, String)> {
    // 这里应该调用服务层来处理业务逻辑，比如保存到数据库
    // 这里只是一个示例，直接返回一个模拟的 Ticket
    let ticket = Ticket {
        id: 1, // 这里应该是数据库生成的 ID
        title: payload.title,
        description: payload.description,
    };
    Ok(Json(ticket))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_ticket,
    ),
    components(
        schemas(
            CreateTicketReq,
            Ticket,
        )
    ),
    tags(
        (name = "Tickets", description = "工单管理模块")
    )
)]
pub struct TicketsApiDoc;

pub fn routes() -> Router {
    Router::new().route("/tickets", post(create_ticket))
}
