//! IT 服务台数据访问层
//!
//! 这个 crate 提供了对 PostgreSQL 数据库的数据访问对象(DAO)实现，
//! 包括工单管理和用户管理功能。

use sqlx::postgres::PgPoolOptions;
use std::env;

pub mod dao;
pub mod models;

/// 创建数据库连接池
pub async fn create_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenvy::dotenv().ok();
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let database = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let jdbc_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&jdbc_url)
        .await
}
