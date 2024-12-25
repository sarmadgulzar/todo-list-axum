use axum::{routing::get, Router};
use sqlx::SqlitePool;

use crate::handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo};

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(pool)
}
