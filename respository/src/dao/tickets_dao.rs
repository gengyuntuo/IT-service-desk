use crate::models::tickets::{CreateTicketParam, TicketModel, TicketPriority, TicketStatus, UpdateTicketParam};
use sqlx::PgPool;
use tracing::instrument;

/// 工单数据访问对象
pub struct TicketsDao {
    pool: PgPool,
}

impl TicketsDao {
    /// 创建新的 TicketsDao 实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 创建新工单
    #[instrument(skip(self))]
    pub async fn create_ticket(&self, param: CreateTicketParam) -> anyhow::Result<TicketModel> {
        let created_ticket = sqlx::query_as!(
            TicketModel,
            r#"
            INSERT INTO it_service.itsd_tickets 
                (title, description, extra_data, attachments, category, status, priority, apply_user_id, approve_user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id, title, description, extra_data, attachments,
                category, 
                status AS "status: TicketStatus", 
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            "#,
            param.title,
            param.description,
            param.extra_data,
            param.attachments,
            param.category,
            param.status as TicketStatus,
            param.priority as TicketPriority,
            param.apply_user_id,
            param.approve_user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_ticket)
    }

    /// 根据 ID 获取工单
    #[instrument(skip(self))]
    pub async fn get_ticket_by_id(&self, id: i64) -> anyhow::Result<Option<TicketModel>> {
        let ticket = sqlx::query_as!(
            TicketModel,
            r#"
            SELECT
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            FROM it_service.itsd_tickets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(ticket)
    }

    /// 获取所有工单列表
    #[instrument(skip(self))]
    pub async fn get_all_tickets(&self) -> anyhow::Result<Vec<TicketModel>> {
        let tickets = sqlx::query_as!(
            TicketModel,
            r#"
            SELECT
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            FROM it_service.itsd_tickets
            ORDER BY id DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    /// 根据申请人 ID 获取工单列表
    #[instrument(skip(self))]
    pub async fn get_tickets_by_apply_user(&self, user_id: i64) -> anyhow::Result<Vec<TicketModel>> {
        let tickets = sqlx::query_as!(
            TicketModel,
            r#"
            SELECT
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            FROM it_service.itsd_tickets
            WHERE apply_user_id = $1
            ORDER BY id DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    /// 根据审批人 ID 获取工单列表
    #[instrument(skip(self))]
    pub async fn get_tickets_by_approve_user(&self, user_id: i64) -> anyhow::Result<Vec<TicketModel>> {
        let tickets = sqlx::query_as!(
            TicketModel,
            r#"
            SELECT
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            FROM it_service.itsd_tickets
            WHERE approve_user_id = $1
            ORDER BY id DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    /// 根据状态获取工单列表
    #[instrument(skip(self))]
    pub async fn get_tickets_by_status(&self, status: TicketStatus) -> anyhow::Result<Vec<TicketModel>> {
        let tickets = sqlx::query_as!(
            TicketModel,
            r#"
            SELECT
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            FROM it_service.itsd_tickets
            WHERE status = $1
            ORDER BY id DESC
            "#,
            status as TicketStatus
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    /// 更新工单
    #[instrument(skip(self))]
    pub async fn update_ticket(&self, param: UpdateTicketParam) -> anyhow::Result<Option<TicketModel>> {
        let updated = sqlx::query_as!(
            TicketModel,
            r#"
            UPDATE it_service.itsd_tickets
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                extra_data = COALESCE($4, extra_data),
                attachments = COALESCE($5, attachments),
                category = COALESCE($6, category),
                status = COALESCE($7::text, status),
                priority = COALESCE($8::text, priority),
                approve_user_id = COALESCE($9, approve_user_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            "#,
            param.id,
            param.title,
            param.description,
            param.extra_data,
            param.attachments,
            param.category,
            param.status.map(|s| s.to_string()),
            param.priority.map(|p| p.to_string()),
            param.approve_user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(updated)
    }

    /// 更新工单状态
    #[instrument(skip(self))]
    pub async fn update_ticket_status(&self, id: i64, status: TicketStatus) -> anyhow::Result<Option<TicketModel>> {
        let updated = sqlx::query_as!(
            TicketModel,
            r#"
            UPDATE it_service.itsd_tickets
            SET
                status = $2,
                updated_at = NOW(),
                finished_at = CASE
                    WHEN $2 IN ('resolved', 'rejected', 'closed') THEN NOW()
                    ELSE NULL
                END
            WHERE id = $1
            RETURNING
                id, title, description, extra_data, attachments,
                category,
                status AS "status: TicketStatus",
                priority AS "priority: TicketPriority",
                apply_user_id, approve_user_id, created_at, updated_at, finished_at
            "#,
            id,
            status as TicketStatus
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(updated)
    }

    /// 删除工单
    #[instrument(skip(self))]
    pub async fn delete_ticket(&self, id: i64) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM it_service.itsd_tickets
            WHERE id = $1
            "#,
            id as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
