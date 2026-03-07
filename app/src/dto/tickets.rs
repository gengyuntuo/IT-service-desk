use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 工单状态枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum TicketStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "closed")]
    Closed,
}

/// 工单优先级枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum TicketPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "urgent")]
    Urgent,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketReq {
    /// 工单标题
    #[schema(example = "Printer not working")]
    pub title: String,

    /// 详细描述
    #[schema(example = "The printer is not printing any documents.")]
    pub description: Option<String>,

    /// 额外的数据，如 JSON 字符串
    #[schema(example = "{\"model\": \"HP LaserJet Pro M404dn\", \"location\": \"Office\"}")]
    pub extra_data: Option<String>,

    /// 附件
    #[schema(example = "base64-encoded-file-content")]
    pub attachments: Option<String>,

    /// 类别：如 "Hardware", "Software", "Network"
    #[schema(example = "Hardware")]
    pub category: String,

    /// 状态：Pending, Approved, Rejected, Closed
    #[schema(example = "Pending")]
    pub status: TicketStatus,

    /// 优先级：Low, Medium, High, Critical
    #[schema(example = "Critical")]
    pub priority: TicketPriority,

    /// 当前审批人/处理人 ID
    #[schema(example = 2)]
    pub approve_user_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketResp {
    /// 唯一标识 ID
    #[schema(example = 1)]
    pub id: i64,

    /// 工单标题
    #[schema(example = "Printer not working")]
    pub title: String,

    /// 详细描述
    #[schema(example = "The printer is not printing any documents.")]
    pub description: Option<String>,

    /// 额外的数据，如 JSON 字符串
    #[schema(example = "{\"model\": \"HP LaserJet Pro M404dn\", \"location\": \"Office\"}")]
    pub extra_data: Option<String>,

    /// 附件
    #[schema(example = "base64-encoded-file-content")]
    pub attachments: Option<String>,

    /// 类别：如 "Hardware", "Software", "Network"
    #[schema(example = "Hardware")]
    pub category: String,

    /// 状态：Pending, Approved, Rejected, Closed
    #[schema(example = "Approved")]
    pub status: TicketStatus,

    /// 优先级：Low, Medium, High, Critical
    #[schema(example = "High")]
    pub priority: TicketPriority,

    /// 申请人用户 ID
    #[schema(example = 1)]
    pub apply_user_id: i64,

    /// 当前审批人/处理人 ID
    #[schema(example = 2)]
    pub approve_user_id: i64,

    /// 创建时间
    #[schema(example = "2024-06-01T12:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,

    /// 最后更新时间
    #[schema(example = "2024-06-01T12:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,

    /// 流程结束时间
    #[schema(example = "2024-06-01T12:00:00Z")]
    pub finished_at: Option<DateTime<Utc>>,
}
