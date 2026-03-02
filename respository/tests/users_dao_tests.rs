//! 用户 DAO 单元测试
//
//! 这个文件包含 UsersDao 的单元测试

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use respository::dao::UsersDao;
    use respository::models::users::{CreateUserRequest, UserRole};

    /// 创建测试用的数据库连接池
    async fn setup_test_pool() -> Result<sqlx::PgPool> {
        // 在测试环境中使用测试数据库
        let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:postgres@localhost:5432/it_service_test".to_string()
        });

        let pool = sqlx::PgPool::connect(&database_url).await?;
        Ok(pool)
    }

    /// 测试创建用户功能
    #[tokio::test]
    async fn test_create_user() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        let create_request = CreateUserRequest {
            username: "test_user_001".to_string(),
            password: "hashed_password_123".to_string(),
            email: "test001@example.com".to_string(),
            role: UserRole::User,
        };

        let user = users_dao
            .create_user(create_request, "hashed_password_123")
            .await?;

        assert_eq!(user.username, "test_user_001");
        assert_eq!(user.email, "test001@example.com");
        // assert_eq!(user.role, UserRole::User);
        assert!(user.id > 0);

        Ok(())
    }

    /// 测试根据用户名获取用户
    #[tokio::test]
    async fn test_get_user_by_username() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        // 先创建一个用户
        let create_request = CreateUserRequest {
            username: "test_user_002".to_string(),
            password: "hashed_password_123".to_string(),
            email: "test002@example.com".to_string(),
            role: UserRole::User,
        };

        let _created_user = users_dao
            .create_user(create_request, "hashed_password_123")
            .await?;

        // 查询用户
        let user = users_dao.get_user_by_username("test_user_002").await?;

        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.username, "test_user_002");
        assert_eq!(user.email, "test002@example.com");

        Ok(())
    }

    /// 测试根据ID获取用户
    #[tokio::test]
    async fn test_get_user_by_id() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        // 先创建一个用户
        let create_request = CreateUserRequest {
            username: "test_user_003".to_string(),
            password: "hashed_password_123".to_string(),
            email: "test003@example.com".to_string(),
            role: UserRole::User,
        };

        let created_user = users_dao
            .create_user(create_request, "hashed_password_123")
            .await?;

        // 根据ID查询用户
        let user = users_dao.get_user_by_id(created_user.id).await?;

        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.id, created_user.id);
        assert_eq!(user.username, "test_user_003");

        Ok(())
    }

    /// 测试更新用户功能
    // #[tokio::test]
    // async fn test_update_user() -> Result<()> {
    //     let pool = setup_test_pool().await?;
    //     let users_dao = UsersDao::new(pool);
    //
    //     // 先创建一个用户
    //     let create_request = CreateUserRequest {
    //         username: "test_user_004".to_string(),
    //         password: "hashed_password_123".to_string(),
    //         email: "test004@example.com".to_string(),
    //         role: UserRole::User,
    //     };
    //
    //     let created_user = users_dao.create_user(create_request, "hashed_password_123").await?;
    //
    //     // 更新用户信息
    //     let update_request = UpdateUserRequest {
    //         username: Some("updated_user_004".to_string()),
    //         password: Some("new_hashed_password".to_string()),
    //         email: Some("updated004@example.com".to_string()),
    //         role: Some(UserRole::Approver),
    //         is_active: Some(true),
    //     };
    //
    //     let updated_user = users_dao.update_user(created_user.id, update_request).await?;
    //
    //     assert!(updated_user.is_some());
    //     let updated_user = updated_user.unwrap();
    //     assert_eq!(updated_user.username, "updated_user_004");
    //     assert_eq!(updated_user.email, "updated004@example.com");
    //     assert_eq!(updated_user.role, UserRole::Approver);
    //
    //     Ok(())
    // }

    /// 测试删除用户功能（软删除）
    #[tokio::test]
    async fn test_delete_user() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        // 先创建一个用户
        let create_request = CreateUserRequest {
            username: "test_user_005".to_string(),
            password: "hashed_password_123".to_string(),
            email: "test005@example.com".to_string(),
            role: UserRole::User,
        };

        let created_user = users_dao
            .create_user(create_request, "hashed_password_123")
            .await?;

        // 删除用户
        let delete_result = users_dao.delete_user(created_user.id).await?;
        assert!(delete_result);

        // 验证用户已被软删除
        let user = users_dao.get_user_by_id(created_user.id).await?;
        assert!(user.is_none());

        Ok(())
    }

    /// 测试获取所有用户
    #[tokio::test]
    async fn test_get_all_users() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        // 清理可能存在的测试数据
        // 注意：在实际测试中，你可能需要清理测试数据

        // 创建几个测试用户
        let usernames = vec!["bulk_user_001", "bulk_user_002", "bulk_user_003"];

        for username in &usernames {
            let create_request = CreateUserRequest {
                username: username.to_string(),
                password: "hashed_password_123".to_string(),
                email: format!("{}@example.com", username),
                role: UserRole::User,
            };
            users_dao
                .create_user(create_request, "hashed_password_123")
                .await?;
        }

        // 获取所有用户
        let users = users_dao.get_all_users().await?;

        // 验证至少有我们创建的用户数量
        assert!(users.len() >= 3);

        // 验证用户名存在
        let user_names: Vec<&str> = users.iter().map(|u| u.username.as_str()).collect();
        for username in &usernames {
            assert!(user_names.contains(username));
        }

        Ok(())
    }

    /// 测试边界情况：查询不存在的用户
    #[tokio::test]
    async fn test_get_nonexistent_user() -> Result<()> {
        let pool = setup_test_pool().await?;
        let users_dao = UsersDao::new(pool);

        // 查询一个肯定不存在的用户ID
        let user = users_dao.get_user_by_id(999999).await?;
        assert!(user.is_none());

        // 查询一个肯定不存在的用户名
        let user = users_dao.get_user_by_username("nonexistent_user").await?;
        assert!(user.is_none());

        Ok(())
    }
}
