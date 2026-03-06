use crate::dto::common::{ApiResponse, PageResponse};
use crate::dto::tickets::{CreateTicketReq, Ticket};
use crate::error::AppError;
use crate::AppState;
use axum::extract::Json;
use axum::routing::post;
use axum::Router;
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/tickets",
    tag = "Tickets",
    request_body = CreateTicketReq,
    responses(
        (status = 200, description = "Ticket created successfully", body = ApiResponse<Ticket>),
    )
)]
pub async fn create_ticket(
    Json(payload): Json<CreateTicketReq>,
) -> Result<ApiResponse<Ticket>, AppError> {
    // 这里应该调用服务层来处理业务逻辑，比如保存到数据库
    // 这里只是一个示例，直接返回一个模拟的 Ticket
    let ticket = Ticket {
        id: 1, // 这里应该是数据库生成的 ID
        title: payload.title,
        description: payload.description,
    };
    Ok(ApiResponse::<Ticket> {
        code: 200,
        data: ticket,
        message: "Ticket created successfully".to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/tickets",
    tag = "Tickets",
    responses(
        (status = 200, description = "List of tickets", body = ApiResponse<PageResponse<Ticket>>),
    )
)]
pub async fn list_tickets() -> Result<ApiResponse<PageResponse<Ticket>>, AppError> {
    // 这里应该调用服务层来处理业务逻辑，比如从数据库中查询数据
    // 这里只是一个示例，直接返回一个模拟的 Ticket 列表
    let tickets = vec![
        Ticket {
            id: 1,
            title: "Ticket 1".to_string(),
            description: Some("Description 1".to_string()),
        },
        Ticket {
            id: 2,
            title: "Ticket 2".to_string(),
            description: Some("Description 2".to_string()),
        },
    ];
    let page_response = PageResponse::<Ticket> {
        total: tickets.len() as u64,
        page_size: 10,
        page: 1,
        total_pages: 1,
        items: tickets,
    };
    Ok(ApiResponse::<PageResponse<Ticket>>::ok(page_response))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_ticket,
        list_tickets,
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

pub fn routers() -> Router<AppState> {
    Router::new().route("/", post(create_ticket))
}
