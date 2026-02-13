use sqlx::PgPool;

/// 工单数据访问对象
pub struct TicketsDao {
    pool: PgPool,
}

impl TicketsDao {
    /// 创建新的TicketsDao实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
