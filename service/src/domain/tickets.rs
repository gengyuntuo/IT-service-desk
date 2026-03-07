use serde::{Deserialize, Serialize};

/// 工单状态枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Rejected,
    Closed,
}

/// 工单优先级枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// 创建工单请求参数（Service 层 DTO）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketDomain {
    /// 工单标题
    pub title: String,
    /// 详细描述
    pub description: Option<String>,
    /// 额外的数据，如 JSON 字符串
    pub extra_data: Option<String>,
    /// 附件
    pub attachments: Option<String>,
    /// 类别：如 "Hardware", "Software", "Network"
    pub category: String,
    /// 状态
    pub status: TicketStatus,
    /// 优先级
    pub priority: TicketPriority,
    /// 申请人用户 ID
    pub apply_user_id: i64,
    /// 当前审批人/处理人 ID
    pub approve_user_id: i64,
}

/// 更新工单请求参数（Service 层 DTO）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketDomain {
    /// 工单 ID
    pub id: i64,
    /// 工单标题（可选）
    pub title: Option<String>,
    /// 详细描述（可选）
    pub description: Option<String>,
    /// 额外的数据，如 JSON 字符串（可选）
    pub extra_data: Option<String>,
    /// 附件（可选）
    pub attachments: Option<String>,
    /// 类别：如 "Hardware", "Software", "Network"（可选）
    pub category: Option<String>,
    /// 状态：Pending, Approved, Rejected, Closed（可选）
    pub status: Option<TicketStatus>,
    /// 优先级：Low, Medium, High, Critical（可选）
    pub priority: Option<TicketPriority>,
    /// 审批人用户 ID（可选）
    pub approve_user_id: Option<i64>,
}

/// 工单响应对象（Service 层 DTO）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDomain {
    /// 唯一标识 ID
    pub id: i64,
    /// 工单标题
    pub title: String,
    /// 详细描述
    pub description: Option<String>,
    /// 额外的数据，如 JSON 字符串
    pub extra_data: Option<String>,
    /// 附件
    pub attachments: Option<String>,
    /// 类别：如 "Hardware", "Software", "Network"
    pub category: String,
    /// 状态
    pub status: TicketStatus,
    /// 优先级
    pub priority: TicketPriority,
    /// 申请人用户 ID
    pub apply_user_id: i64,
    /// 当前审批人/处理人 ID
    pub approve_user_id: i64,
    /// 创建时间
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 最后更新时间
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// 流程结束时间
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
}
