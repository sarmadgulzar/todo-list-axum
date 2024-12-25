use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
