use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户角色枚举
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "approver")]
    Approver,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Approver => write!(f, "approver"),
        }
    }
}

/// 用户实体结构体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// 唯一标识 ID
    pub id: i32,

    /// 用户名
    pub username: String,

    /// 密码哈希值
    pub password_hashed: String,

    /// 邮箱地址
    pub email: String,

    /// 用户角色
    pub role: UserRole,

    /// 是否激活
    pub is_active: bool,

    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,

    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 创建用户请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户名
    pub username: String,

    /// 密码（明文）
    pub password: String,

    /// 邮箱地址
    pub email: String,

    /// 用户角色
    pub role: UserRole,
}

/// 更新用户请求结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// 用户名（可选）
    pub username: Option<String>,

    /// 密码（可选，明文）
    pub password: Option<String>,

    /// 邮箱地址（可选）
    pub email: Option<String>,

    /// 用户角色（可选）
    pub role: Option<UserRole>,

    /// 是否激活（可选）
    pub is_active: Option<bool>,
}
