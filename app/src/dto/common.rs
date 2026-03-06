use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

/// 统一 API 响应结构
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: ToSchema + Serialize> {
    /// 状态码
    #[schema(example = 0)]
    pub code: u32,
    /// 响应消息
    #[schema(example = "Success")]
    pub message: String,
    /// 响应数据
    pub data: T,
}

impl ApiResponse<()> {
    pub fn new(code: u32, message: String) -> Self {
        Self {
            code,
            message,
            data: (),
        }
    }
}

impl ApiResponse<()> {
    /// 错误响应
    pub fn error(code: u32, message: String) -> Self {
        Self {
            code,
            message,
            data: (),
        }
    }
}

impl<T: ToSchema + Serialize> ApiResponse<T> {
    /// 成功响应
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "Success".to_string(),
            data,
        }
    }
}

impl<T: ToSchema + Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // 这里你可以决定如何渲染这个对象
        // 比如：默认转为 JSON，或者转为 HTML，或者根据情况返回不同状态码
        Json(self).into_response()
    }
}

/// 分页响应
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T: Serialize + ToSchema> {
    /// 总记录数
    #[schema(example = 100)]
    pub total: u64,
    /// 总页数
    #[schema(example = 5)]
    pub total_pages: u32,
    /// 当前页码
    #[schema(example = 1)]
    pub page: u32,
    /// 每页记录数
    #[schema(example = 20)]
    pub page_size: u32,
    /// 数据列表
    pub items: Vec<T>,
}
