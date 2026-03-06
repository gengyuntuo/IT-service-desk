use crate::dto::common::{ApiResponse, PageResponse};
use crate::dto::users::{CreateUserReq, UpdateUserReq, User};
use crate::error::AppError;
use crate::{dto, AppState};
use axum::extract::{Json, Path};
use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUserReq,
    responses(
        (status = 200, description = "User created successfully", body = ApiResponse<User>),
    )
)]
pub async fn create_user(
    Json(payload): Json<CreateUserReq>,
) -> Result<ApiResponse<User>, AppError> {
    let user = User {
        id: 1,
        username: payload.username,
        email: payload.email,
        role: payload.role,
        is_active: true,
        created_at: None,
        updated_at: None,
    };
    Ok(ApiResponse::<User>::ok(user))
}

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "List of users", body = ApiResponse<PageResponse<User>>),
    )
)]
pub async fn list_users() -> Result<ApiResponse<PageResponse<User>>, AppError> {
    let users = vec![
        User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: dto::users::UserRole::Admin,
            is_active: true,
            created_at: None,
            updated_at: None,
        },
        User {
            id: 2,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            role: dto::users::UserRole::User,
            is_active: true,
            created_at: None,
            updated_at: None,
        },
    ];
    let page_response = PageResponse::<User> {
        total: users.len() as u64,
        page_size: 10,
        page: 1,
        total_pages: 1,
        items: users,
    };
    Ok(ApiResponse::<PageResponse<User>>::ok(page_response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, description = "用户 ID"),
    ),
    responses(
        (status = 200, description = "User details", body = ApiResponse<User>),
        (status = 404, description = "User not found"),
    )
)]
pub async fn get_user(Path(id): Path<i32>) -> Result<ApiResponse<User>, AppError> {
    let user = User {
        id,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: dto::users::UserRole::Admin,
        is_active: true,
        created_at: None,
        updated_at: None,
    };
    Ok(ApiResponse::<User>::ok(user))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, description = "用户 ID"),
    ),
    request_body = UpdateUserReq,
    responses(
        (status = 200, description = "User updated successfully", body = ApiResponse<User>),
        (status = 404, description = "User not found"),
    )
)]
pub async fn update_user(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserReq>,
) -> Result<ApiResponse<User>, AppError> {
    let user = User {
        id,
        username: payload.username.unwrap_or_else(|| "admin".to_string()),
        email: payload
            .email
            .unwrap_or_else(|| "admin@example.com".to_string()),
        role: payload.role.unwrap_or(dto::users::UserRole::User),
        is_active: payload.is_active.unwrap_or(true),
        created_at: None,
        updated_at: None,
    };
    Ok(ApiResponse::<User>::ok(user))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, description = "用户 ID"),
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn delete_user(Path(id): Path<i32>) -> Result<ApiResponse<()>, AppError> {
    Ok(ApiResponse::<()>::ok(()))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        list_users,
        get_user,
        update_user,
        delete_user,
    ),
    components(
        schemas(
            CreateUserReq,
            UpdateUserReq,
            User,
        )
    ),
    tags(
        (name = "Users", description = "用户管理模块")
    )
)]
pub struct UsersApiDoc;

pub fn routers() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}
