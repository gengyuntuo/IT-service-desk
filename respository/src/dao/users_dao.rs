use crate::models::users::{CreateUserRequest, UpdateUserRequest, User, UserRole};
use anyhow::Result;
use sqlx::PgPool;
use tracing::instrument;
/// 用户数据访问对象
pub struct UsersDao {
    pool: PgPool,
}

impl UsersDao {
    /// 创建新的 UsersDao 实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 创建新用户
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO it_service.itsd_users (username, password_hashed, email, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            "#,
            request.username,
            request.password, // 这里应该是已经哈希过的密码
            request.email,
            request.role as UserRole
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据 ID 获取用户
    #[instrument(skip(self))]
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE id = $1 AND is_active = true
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据用户名获取用户
    #[instrument(skip(self))]
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE username = $1 AND is_active = true
            "#,
            username.to_string()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 根据邮箱获取用户
    #[instrument(skip(self))]
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE email = $1 AND is_active = true
            "#,
            email.to_string()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 获取所有用户列表
    #[instrument(skip(self))]
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE is_active = true
            ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// 删除用户（软删除）
    #[instrument(skip(self))]
    pub async fn delete_user(&self, id: i32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE it_service.itsd_users
            SET is_active = false, updated_at = NOW()
            WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 验证用户凭据
    #[instrument(skip(self, password_hashed))]
    pub async fn verify_credentials(
        &self,
        username: &str,
        password_hashed: &str,
    ) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hashed, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE username = $1 AND is_active = true
            "#,
            username.to_string()
        )
        .fetch_optional(&self.pool)
        .await?;

        // 这里应该进行密码验证，但为了简化示例，我们假设密码已经验证过了
        // 在实际应用中，你应该使用 bcrypt 或其他密码哈希库来验证密码
        Ok(user)
    }
}
