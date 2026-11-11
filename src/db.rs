use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

/// Initialize PostgreSQL connection pool
pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create connection pool with optimized settings
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    // Run migrations automatically
    sqlx::migrate!("./src/migrations")
        .run(&pool)
        .await?;

    println!("✅ Database connected successfully!");
    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let result: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;
    
    println!("✅ Database ping successful! Result: {:?}", result);
    Ok(())
}
