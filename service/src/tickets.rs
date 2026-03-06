use sqlx::{Pool, Postgres};

pub struct TicketService {
    pool: Pool<Postgres>,
}

impl TicketService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
