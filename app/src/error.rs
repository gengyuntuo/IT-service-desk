use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

// 统一错误响应结构
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    // path: String, // 可选：记录路径
}

// 定义你的全局错误枚举
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    // 包装第三方库错误
    SqlxError(sqlx::Error),
    SerdeError(serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::SqlxError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operation failed".to_string(),
            ),
            AppError::SerdeError(err) => (
                StatusCode::BAD_REQUEST,
                "Invalid request format".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            code: status.as_u16(),
            message,
        });

        (status, body).into_response()
    }
}

// 实现 From 特质，让 ? 操作符自动转换错误
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::SqlxError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerdeError(err)
    }
}
