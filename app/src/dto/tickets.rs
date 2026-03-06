use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketReq {
    /// 标题
    pub title: String,
    /// 描述
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
}
