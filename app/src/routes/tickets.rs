use crate::dto::common::{ApiResponse, PageResponse};
use crate::dto::tickets::{CreateTicketReq, TicketPriority, TicketResp, TicketStatus};
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
        (status = 200, description = "Ticket created successfully", body = ApiResponse<TicketResp>),
    )
)]
pub async fn create_ticket(
    Json(payload): Json<CreateTicketReq>,
) -> Result<ApiResponse<TicketResp>, AppError> {
    // TODO: 调用服务层来处理业务逻辑
    let ticket = TicketResp {
        id: 1,
        title: payload.title,
        description: payload.description,
        extra_data: None,
        attachments: None,
        category: "General".to_string(),
        status: TicketStatus::Open,
        priority: TicketPriority::Medium,
        apply_user_id: 1,
        approve_user_id: 1,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
        finished_at: None,
    };
    Ok(ApiResponse::<TicketResp> {
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
        (status = 200, description = "List of tickets", body = ApiResponse<PageResponse<TicketResp>>),
    )
)]
pub async fn list_tickets() -> Result<ApiResponse<PageResponse<TicketResp>>, AppError> {
    // TODO: 调用服务层来处理业务逻辑
    let tickets = vec![
        TicketResp {
            id: 1,
            title: "Ticket 1".to_string(),
            description: Some("Description 1".to_string()),
            extra_data: None,
            attachments: None,
            category: "General".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 1,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            finished_at: None,
        },
        TicketResp {
            id: 2,
            title: "Ticket 2".to_string(),
            description: Some("Description 2".to_string()),
            extra_data: None,
            attachments: None,
            category: "General".to_string(),
            status: TicketStatus::InProgress,
            priority: TicketPriority::High,
            apply_user_id: 1,
            approve_user_id: 2,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            finished_at: None,
        },
    ];
    let page_response = PageResponse::<TicketResp> {
        total: tickets.len() as u64,
        page_size: 10,
        page: 1,
        total_pages: 1,
        items: tickets,
    };
    Ok(ApiResponse::<PageResponse<TicketResp>>::ok(page_response))
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
            TicketResp,
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
