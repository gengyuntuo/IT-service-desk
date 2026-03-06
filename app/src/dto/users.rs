use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 用户角色枚举
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "approver")]
    Approver,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserReq {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: String,
    /// 角色
    pub role: UserRole,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserReq {
    /// 用户名（可选）
    pub username: Option<String>,
    /// 密码（可选）
    pub password: Option<String>,
    /// 邮箱（可选）
    pub email: Option<String>,
    /// 角色（可选）
    pub role: Option<UserRole>,
    /// 是否激活（可选）
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    #[schema(value_type = String)]
    pub created_at: Option<DateTime<Utc>>,
    #[schema(value_type = String)]
    pub updated_at: Option<DateTime<Utc>>,
}
