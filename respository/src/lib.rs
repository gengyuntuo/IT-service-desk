use sqlx::postgres::PgPoolOptions;
use std::env;

mod dao;
mod models;

#[tokio::test]
async fn it_works() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok(); // .ok() 忽略错误（如文件不存在）
    // 现在可以从 env 读取
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let database = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let jdbc_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&jdbc_url)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    Ok(())
}
