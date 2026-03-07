use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 工单状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
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

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketStatus::Open => write!(f, "open"),
            TicketStatus::InProgress => write!(f, "in_progress"),
            TicketStatus::Resolved => write!(f, "resolved"),
            TicketStatus::Rejected => write!(f, "rejected"),
            TicketStatus::Closed => write!(f, "closed"),
        }
    }
}

/// 工单优先级枚举
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
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

impl std::fmt::Display for TicketPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketPriority::Low => write!(f, "low"),
            TicketPriority::Medium => write!(f, "medium"),
            TicketPriority::High => write!(f, "high"),
            TicketPriority::Urgent => write!(f, "urgent"),
        }
    }
}

/// 工单实体结构体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    /// 唯一标识 ID
    pub id: i64,

    /// 工单标题
    pub title: String,

    /// 详细描述
    pub description: Option<String>,

    /// 附件地址或 JSON 字符串
    pub attachments: Option<String>,

    /// 类别：如 "Hardware", "Software", "Network"
    pub category: String,

    /// 状态：Pending, Approved, Rejected, Closed
    pub status: TicketStatus,

    /// 优先级：Low, Medium, High, Critical
    pub priority: TicketPriority,

    /// 申请人用户 ID
    pub apply_user_id: i64,

    /// 当前审批人/处理人 ID
    pub approve_user_id: i64,

    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,

    /// 最后更新时间
    pub updated_at: Option<DateTime<Utc>>,

    /// 流程结束时间
    pub finished_at: Option<DateTime<Utc>>,
}
