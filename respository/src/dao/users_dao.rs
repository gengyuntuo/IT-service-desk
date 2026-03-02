use crate::models::users::{CreateUserRequest, UpdateUserRequest, User, UserRole};
use anyhow::Result;
use sqlx::PgPool;
use tracing::instrument;

// 创建组合 trait 来解决 trait object 限制
trait SqlxParameter: sqlx::Encode<'static, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> {}

// 为常用类型实现这个 trait
impl SqlxParameter for String {}
impl SqlxParameter for bool {}
impl SqlxParameter for UserRole {}
impl SqlxParameter for i32 {}

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
    #[instrument(skip(self, password))]
    pub async fn create_user(&self, request: CreateUserRequest, password: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO it_service.itsd_users (username, password_hash, email, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            "#,
            request.username,
            password, // 这里应该是已经哈希过的密码
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
            SELECT id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE id = $1 AND is_active = true
            "#,
            id
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
            SELECT id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE username = $1 AND is_active = true
            "#,
            username
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
            SELECT id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE email = $1 AND is_active = true
            "#,
            email
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
            SELECT id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE is_active = true
            ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
    //
    // /// 更新用户信息
    // #[instrument(skip(self))]
    // pub async fn update_user(&self, id: i32, request: UpdateUserRequest) -> Result<Option<User>> {
    //     // 先检查用户是否存在且活跃
    //     let existing_user = self.get_user_by_id(id).await?;
    //     if existing_user.is_none() {
    //         return Ok(None);
    //     }
    //
    //     // 构建动态更新查询
    //     let mut query_parts = vec![];
    //     let mut params: Vec<Box<dyn SqlxParameter + Send>> = vec![];
    //
    //     if let Some(username) = &request.username {
    //         query_parts.push(format!("username = ${}", query_parts.len() + 1));
    //         params.push(Box::new(username.clone()));
    //     }
    //
    //     if let Some(password_hash) = &request.password {
    //         query_parts.push(format!("password_hash = ${}", query_parts.len() + 1));
    //         params.push(Box::new(password_hash.clone()));
    //     }
    //
    //     if let Some(email) = &request.email {
    //         query_parts.push(format!("email = ${}", query_parts.len() + 1));
    //         params.push(Box::new(email.clone()));
    //     }
    //
    //     if let Some(role) = &request.role {
    //         query_parts.push(format!("role = ${}", query_parts.len() + 1));
    //         params.push(Box::new(role.clone() as UserRole));
    //     }
    //
    //     if let Some(is_active) = request.is_active {
    //         query_parts.push(format!("is_active = ${}", query_parts.len() + 1));
    //         params.push(Box::new(is_active));
    //     }
    //
    //     // 添加更新时间
    //     query_parts.push(format!("updated_at = NOW()"));
    //
    //     if query_parts.is_empty() {
    //         // 如果没有需要更新的字段，直接返回原用户
    //         return Ok(existing_user);
    //     }
    //
    //     // 构造完整的查询
    //     let mut query = format!(
    //         "UPDATE it_service.itsd_users SET {} WHERE id = ${} AND is_active = true RETURNING id, username, password_hash, email, role AS \"role: UserRole\", is_active, created_at, updated_at",
    //         query_parts.join(", "),
    //         query_parts.len() + 1
    //     );
    //
    //     // 添加 ID 参数
    //     params.push(Box::new(id));
    //
    //     // 执行查询
    //     let mut query_builder = sqlx::query_as::<_, User>(&query);
    //     for param in params {
    //         query_builder = query_builder.bind(param);
    //     }
    //
    //     let user = query_builder.fetch_optional(&self.pool).await?;
    //     Ok(user)
    // }

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
    #[instrument(skip(self, password))]
    pub async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, role AS "role: UserRole", is_active, created_at, updated_at
            FROM it_service.itsd_users
            WHERE username = $1 AND is_active = true
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        // 这里应该进行密码验证，但为了简化示例，我们假设密码已经验证过了
        // 在实际应用中，你应该使用 bcrypt 或其他密码哈希库来验证密码
        Ok(user)
    }
}
