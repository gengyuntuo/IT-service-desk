//! 工单 DAO 单元测试
//! 这个文件包含 TicketsDao 的单元测试

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use dotenvy::dotenv;
    use respository::dao::TicketsDao;
    use respository::models::tickets::{
        CreateTicketParam, TicketPriority, TicketStatus, UpdateTicketParam,
    };
    use std::env;

    /// 创建测试用的数据库连接池
    async fn setup_test_pool() -> Result<sqlx::PgPool> {
        if let Err(e) = dotenv() {
            eprintln!("Failed to load .env file: {}", e);
            panic!("Failed to load .env file");
        }
        // 读取环境变量
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::PgPool::connect(&database_url).await?;
        Ok(pool)
    }

    /// 测试创建工单功能
    #[tokio::test]
    async fn test_create_ticket() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        let create_param = CreateTicketParam {
            title: "Test Ticket".to_string(),
            description: Some("This is a test ticket".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        let ticket = tickets_dao.create_ticket(create_param).await?;

        assert_eq!(ticket.title, "Test Ticket");
        assert_eq!(ticket.description.unwrap(), "This is a test ticket");
        assert_eq!(ticket.category, "Hardware");
        assert_eq!(ticket.status, TicketStatus::Open);
        assert_eq!(ticket.priority, TicketPriority::Medium);
        assert_eq!(ticket.apply_user_id, 1);
        assert_eq!(ticket.approve_user_id, 2);
        assert!(ticket.id > 0);

        Ok(())
    }

    /// 测试根据 ID 获取工单
    #[tokio::test]
    async fn test_get_ticket_by_id() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 先创建一个工单
        let create_param = CreateTicketParam {
            title: "Test Ticket for Get".to_string(),
            description: Some("Description for get test".to_string()),
            extra_data: None,
            attachments: None,
            category: "Software".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::High,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        let created_ticket = tickets_dao.create_ticket(create_param).await?;

        // 根据 ID 查询工单
        let ticket = tickets_dao.get_ticket_by_id(created_ticket.id).await?;

        assert!(ticket.is_some());
        let ticket = ticket.unwrap();
        assert_eq!(ticket.id, created_ticket.id);
        assert_eq!(ticket.title, "Test Ticket for Get");

        Ok(())
    }

    /// 测试更新工单功能
    #[tokio::test]
    async fn test_update_ticket() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 先创建一个工单
        let create_param = CreateTicketParam {
            title: "Original Title".to_string(),
            description: Some("Original description".to_string()),
            extra_data: None,
            attachments: None,
            category: "Network".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Low,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        let created_ticket = tickets_dao.create_ticket(create_param).await?;

        // 更新工单（部分字段）
        let update_param = UpdateTicketParam {
            id: created_ticket.id,
            title: Some("Updated Title".to_string()),
            description: Some("Updated description".to_string()),
            extra_data: None,
            attachments: None,
            category: None,
            status: Some(TicketStatus::InProgress),
            priority: Some(TicketPriority::High),
            approve_user_id: Some(3),
        };

        let updated_ticket = tickets_dao.update_ticket(update_param).await?;

        assert!(updated_ticket.is_some());
        let updated_ticket = updated_ticket.unwrap();
        assert_eq!(updated_ticket.title, "Updated Title");
        assert_eq!(updated_ticket.description.unwrap(), "Updated description");
        assert_eq!(updated_ticket.category, "Network"); // 未更新，保持原值
        assert_eq!(updated_ticket.status, TicketStatus::InProgress);
        assert_eq!(updated_ticket.priority, TicketPriority::High);
        assert_eq!(updated_ticket.approve_user_id, 3);

        Ok(())
    }

    /// 测试获取所有工单
    #[tokio::test]
    async fn test_get_all_tickets() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 创建几个测试工单
        let titles = vec!["Ticket 001", "Ticket 002", "Ticket 003"];

        for title in &titles {
            let create_param = CreateTicketParam {
                title: title.to_string(),
                description: Some(format!("Description for {}", title)),
                extra_data: None,
                attachments: None,
                category: "Hardware".to_string(),
                status: TicketStatus::Open,
                priority: TicketPriority::Medium,
                apply_user_id: 1,
                approve_user_id: 2,
            };
            tickets_dao.create_ticket(create_param).await?;
        }

        // 获取所有工单
        let tickets = tickets_dao.get_all_tickets().await?;

        // 验证至少有我们创建的工单数量
        assert!(tickets.len() >= 3);

        // 验证标题存在
        let ticket_titles: Vec<&str> = tickets.iter().map(|t| t.title.as_str()).collect();
        for title in &titles {
            assert!(ticket_titles.contains(title));
        }

        Ok(())
    }

    /// 测试根据申请人 ID 获取工单
    #[tokio::test]
    async fn test_get_tickets_by_apply_user() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 创建测试工单
        let create_param = CreateTicketParam {
            title: "User Test Ticket".to_string(),
            description: Some("Test for apply user".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 999,
            approve_user_id: 2,
        };

        tickets_dao.create_ticket(create_param).await?;

        // 根据申请人 ID 查询
        let tickets = tickets_dao.get_tickets_by_apply_user(999).await?;

        assert!(tickets.len() >= 1);
        assert!(tickets.iter().all(|t| t.apply_user_id == 999));

        Ok(())
    }

    /// 测试根据审批人 ID 获取工单
    #[tokio::test]
    async fn test_get_tickets_by_approve_user() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 创建测试工单
        let create_param = CreateTicketParam {
            title: "Approver Test Ticket".to_string(),
            description: Some("Test for approve user".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 888,
        };

        tickets_dao.create_ticket(create_param).await?;

        // 根据审批人 ID 查询
        let tickets = tickets_dao.get_tickets_by_approve_user(888).await?;

        assert!(tickets.len() >= 1);
        assert!(tickets.iter().all(|t| t.approve_user_id == 888));

        Ok(())
    }

    /// 测试根据状态获取工单
    #[tokio::test]
    async fn test_get_tickets_by_status() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 创建测试工单
        let create_param = CreateTicketParam {
            title: "Status Test Ticket".to_string(),
            description: Some("Test for status".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Resolved,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        tickets_dao.create_ticket(create_param).await?;

        // 根据状态查询
        let tickets = tickets_dao
            .get_tickets_by_status(TicketStatus::Resolved)
            .await?;

        assert!(tickets.len() >= 1);
        assert!(tickets.iter().all(|t| t.status == TicketStatus::Resolved));

        Ok(())
    }

    /// 测试更新工单状态
    #[tokio::test]
    async fn test_update_ticket_status() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 先创建一个工单
        let create_param = CreateTicketParam {
            title: "Status Update Test".to_string(),
            description: Some("Test for status update".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        let created_ticket = tickets_dao.create_ticket(create_param).await?;

        // 更新状态
        let updated_ticket = tickets_dao
            .update_ticket_status(created_ticket.id, TicketStatus::Resolved)
            .await?;

        assert!(updated_ticket.is_some());
        let updated_ticket = updated_ticket.unwrap();
        assert_eq!(updated_ticket.status, TicketStatus::Resolved);
        assert!(updated_ticket.finished_at.is_some());

        Ok(())
    }

    /// 测试删除工单功能
    #[tokio::test]
    async fn test_delete_ticket() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 先创建一个工单
        let create_param = CreateTicketParam {
            title: "Ticket to Delete".to_string(),
            description: Some("This ticket will be deleted".to_string()),
            extra_data: None,
            attachments: None,
            category: "Hardware".to_string(),
            status: TicketStatus::Open,
            priority: TicketPriority::Medium,
            apply_user_id: 1,
            approve_user_id: 2,
        };

        let created_ticket = tickets_dao.create_ticket(create_param).await?;

        // 删除工单
        let delete_result = tickets_dao.delete_ticket(created_ticket.id).await?;
        assert!(delete_result);

        // 验证工单已被删除
        let ticket = tickets_dao.get_ticket_by_id(created_ticket.id).await?;
        assert!(ticket.is_none());

        Ok(())
    }

    /// 测试边界情况：查询不存在的工单
    #[tokio::test]
    async fn test_get_nonexistent_ticket() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 查询一个肯定不存在的工单 ID
        let ticket = tickets_dao.get_ticket_by_id(999999).await?;
        assert!(ticket.is_none());

        Ok(())
    }

    /// 测试更新不存在的工单
    #[tokio::test]
    async fn test_update_nonexistent_ticket() -> Result<()> {
        let pool = setup_test_pool().await?;
        let tickets_dao = TicketsDao::new(pool);

        // 尝试更新一个不存在的工单
        let update_param = UpdateTicketParam {
            id: 999999,
            title: Some("Non-existent Title".to_string()),
            description: None,
            extra_data: None,
            attachments: None,
            category: None,
            status: None,
            priority: None,
            approve_user_id: None,
        };

        let result = tickets_dao.update_ticket(update_param).await?;
        assert!(result.is_none());

        Ok(())
    }
}
