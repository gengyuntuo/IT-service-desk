use sqlx::{Pool, Postgres};
use crate::domain::tickets::{CreateTicketDomain, TicketDomain, TicketPriority as ServiceTicketPriority, TicketStatus as ServiceTicketStatus, UpdateTicketDomain};

/// 工单服务层
pub struct TicketService {
    pool: Pool<Postgres>,
    ticket_dao: respository::dao::TicketsDao,
}

impl TicketService {
    /// 创建新的工单服务实例
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool: pool.clone(),
            ticket_dao: respository::dao::TicketsDao::new(pool.clone()),
        }
    }

    /// 创建新工单
    pub async fn create_ticket(
        &self,
        dto: CreateTicketDomain,
    ) -> anyhow::Result<TicketDomain> {
        // 将 Service 层 DTO 转换为 Repository 层 Param
        let param = respository::models::tickets::CreateTicketParam {
            title: dto.title,
            description: dto.description,
            extra_data: dto.extra_data,
            attachments: dto.attachments,
            category: dto.category,
            status: match dto.status {
                ServiceTicketStatus::Open => respository::models::tickets::TicketStatus::Open,
                ServiceTicketStatus::InProgress => respository::models::tickets::TicketStatus::InProgress,
                ServiceTicketStatus::Resolved => respository::models::tickets::TicketStatus::Resolved,
                ServiceTicketStatus::Rejected => respository::models::tickets::TicketStatus::Rejected,
                ServiceTicketStatus::Closed => respository::models::tickets::TicketStatus::Closed,
            },
            priority: match dto.priority {
                ServiceTicketPriority::Low => respository::models::tickets::TicketPriority::Low,
                ServiceTicketPriority::Medium => respository::models::tickets::TicketPriority::Medium,
                ServiceTicketPriority::High => respository::models::tickets::TicketPriority::High,
                ServiceTicketPriority::Urgent => respository::models::tickets::TicketPriority::Urgent,
            },
            apply_user_id: dto.apply_user_id,
            approve_user_id: dto.approve_user_id,
        };

        let model = self.ticket_dao.create_ticket(param).await?;
        Ok(Self::model_to_dto(model))
    }

    /// 根据 ID 获取工单
    pub async fn get_ticket_by_id(&self, id: i64) -> anyhow::Result<Option<TicketDomain>> {
        let model = self.ticket_dao.get_ticket_by_id(id).await?;
        Ok(model.map(Self::model_to_dto))
    }

    /// 获取所有工单列表
    pub async fn get_all_tickets(&self) -> anyhow::Result<Vec<TicketDomain>> {
        let models = self.ticket_dao.get_all_tickets().await?;
        Ok(models.into_iter().map(Self::model_to_dto).collect())
    }

    /// 根据申请人 ID 获取工单列表
    pub async fn get_tickets_by_apply_user(&self, user_id: i64) -> anyhow::Result<Vec<TicketDomain>> {
        let models = self.ticket_dao.get_tickets_by_apply_user(user_id).await?;
        Ok(models.into_iter().map(Self::model_to_dto).collect())
    }

    /// 根据审批人 ID 获取工单列表
    pub async fn get_tickets_by_approve_user(&self, user_id: i64) -> anyhow::Result<Vec<TicketDomain>> {
        let models = self.ticket_dao.get_tickets_by_approve_user(user_id).await?;
        Ok(models.into_iter().map(Self::model_to_dto).collect())
    }

    /// 根据状态获取工单列表
    pub async fn get_tickets_by_status(&self, status: ServiceTicketStatus) -> anyhow::Result<Vec<TicketDomain>> {
        let db_status = match status {
            ServiceTicketStatus::Open => respository::models::tickets::TicketStatus::Open,
            ServiceTicketStatus::InProgress => respository::models::tickets::TicketStatus::InProgress,
            ServiceTicketStatus::Resolved => respository::models::tickets::TicketStatus::Resolved,
            ServiceTicketStatus::Rejected => respository::models::tickets::TicketStatus::Rejected,
            ServiceTicketStatus::Closed => respository::models::tickets::TicketStatus::Closed,
        };
        let models = self.ticket_dao.get_tickets_by_status(db_status).await?;
        Ok(models.into_iter().map(Self::model_to_dto).collect())
    }

    /// 更新工单（支持部分字段更新）
    pub async fn update_ticket(
        &self,
        dto: UpdateTicketDomain,
    ) -> anyhow::Result<Option<TicketDomain>> {
        // 将 Service 层 DTO 转换为 Repository 层 Param
        let param = respository::models::tickets::UpdateTicketParam {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            extra_data: dto.extra_data,
            attachments: dto.attachments,
            category: dto.category,
            status: dto.status.map(|s| match s {
                ServiceTicketStatus::Open => respository::models::tickets::TicketStatus::Open,
                ServiceTicketStatus::InProgress => respository::models::tickets::TicketStatus::InProgress,
                ServiceTicketStatus::Resolved => respository::models::tickets::TicketStatus::Resolved,
                ServiceTicketStatus::Rejected => respository::models::tickets::TicketStatus::Rejected,
                ServiceTicketStatus::Closed => respository::models::tickets::TicketStatus::Closed,
            }),
            priority: dto.priority.map(|p| match p {
                ServiceTicketPriority::Low => respository::models::tickets::TicketPriority::Low,
                ServiceTicketPriority::Medium => respository::models::tickets::TicketPriority::Medium,
                ServiceTicketPriority::High => respository::models::tickets::TicketPriority::High,
                ServiceTicketPriority::Urgent => respository::models::tickets::TicketPriority::Urgent,
            }),
            approve_user_id: dto.approve_user_id,
        };

        let model = self.ticket_dao.update_ticket(param).await?;
        Ok(model.map(Self::model_to_dto))
    }

    /// 更新工单状态
    pub async fn update_ticket_status(
        &self,
        id: i64,
        status: ServiceTicketStatus,
    ) -> anyhow::Result<Option<TicketDomain>> {
        let db_status = match status {
            ServiceTicketStatus::Open => respository::models::tickets::TicketStatus::Open,
            ServiceTicketStatus::InProgress => respository::models::tickets::TicketStatus::InProgress,
            ServiceTicketStatus::Resolved => respository::models::tickets::TicketStatus::Resolved,
            ServiceTicketStatus::Rejected => respository::models::tickets::TicketStatus::Rejected,
            ServiceTicketStatus::Closed => respository::models::tickets::TicketStatus::Closed,
        };
        let model = self.ticket_dao.update_ticket_status(id, db_status).await?;
        Ok(model.map(Self::model_to_dto))
    }

    /// 删除工单
    pub async fn delete_ticket(&self, id: i64) -> anyhow::Result<bool> {
        self.ticket_dao.delete_ticket(id).await
    }

    /// 将 Repository 层的 Model 转换为 Service 层的 DTO
    fn model_to_dto(model: respository::models::tickets::TicketModel) -> TicketDomain {
        TicketDomain {
            id: model.id,
            title: model.title,
            description: model.description,
            extra_data: model.extra_data,
            attachments: model.attachments,
            category: model.category,
            status: match model.status {
                respository::models::tickets::TicketStatus::Open => ServiceTicketStatus::Open,
                respository::models::tickets::TicketStatus::InProgress => ServiceTicketStatus::InProgress,
                respository::models::tickets::TicketStatus::Resolved => ServiceTicketStatus::Resolved,
                respository::models::tickets::TicketStatus::Rejected => ServiceTicketStatus::Rejected,
                respository::models::tickets::TicketStatus::Closed => ServiceTicketStatus::Closed,
            },
            priority: match model.priority {
                respository::models::tickets::TicketPriority::Low => ServiceTicketPriority::Low,
                respository::models::tickets::TicketPriority::Medium => ServiceTicketPriority::Medium,
                respository::models::tickets::TicketPriority::High => ServiceTicketPriority::High,
                respository::models::tickets::TicketPriority::Urgent => ServiceTicketPriority::Urgent,
            },
            apply_user_id: model.apply_user_id,
            approve_user_id: model.approve_user_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            finished_at: model.finished_at,
        }
    }
}
