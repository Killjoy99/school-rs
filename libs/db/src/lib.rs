use std::env;

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