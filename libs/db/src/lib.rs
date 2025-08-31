use std::env;
use anyhow::Result;
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub fn db_connection_string() -> String {
    format!("mysql://{}:{}@{}:{}/{}",
        env::var("DATABASE_USER").unwrap_or_else(|_| "root".to_string()), // Default to root user
        env::var("DATABASE_PASSWORD").unwrap_or_default(), // Default to empty password
        env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string()),  // Default to localhost
        env::var("DATABASE_PORT").unwrap_or_else(|_| "3306".to_string()),       // Default to 3306
        env::var("DATABASE_NAME").unwrap_or_else(|_| "school".to_string())   // Default to school_rs database
    )
}

pub fn redis_connection_string() -> String {
    format!("redis://{}:{}@{}:{}/{}",
        env::var("REDIS_USER").unwrap_or_default(), // Default to empty user
        env::var("REDIS_PASSWORD").unwrap_or_default(), // Default to empty password
        env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),  // Default to localhost
        env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string()),       // Default to 6379
        env::var("REDIS_DB").unwrap_or_else(|_| "0".to_string())            // Default to DB 0
    )
}

pub async fn init_db() -> Result<Pool<MySql>> {
    let db_url = db_connection_string();

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run migrations from root workspace
    sqlx::migrate!("../../migrations").run(&pool).await?;

    println!("Database connected and migrations applied.");
    
    Ok(pool)
}

// Optional: Add helper functions for transactions, queries, etc.
pub async fn execute_migration(pool: &Pool<MySql>, migration_file: &str) -> Result<()> {
    let migration_sql = std::fs::read_to_string(format!("../../migrations/{}", migration_file))?;
    sqlx::query(&migration_sql).execute(pool).await?;
    Ok(())
}