use dotenvy;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

fn get_database_url() -> String {
    dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set in .env")
}

pub async fn initialize_db() -> SqlitePool {
    let database_url = get_database_url();

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(&database_url).await.unwrap();
    }

    SqlitePool::connect(&database_url).await.unwrap()
}
