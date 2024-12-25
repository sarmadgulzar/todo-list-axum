use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    error::{AppError, ErrorResponse},
    models::{CreateTodo, Todo, UpdateTodo},
};

pub async fn list_todos(State(pool): State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, title, completed FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(todos)
}

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                detail: "Title cannot be empty".to_string(),
            }),
        ));
    }

    let todo = sqlx::query_as!(
        Todo,
        r#"INSERT INTO todos (title, completed) VALUES (?1, false) RETURNING id, title, completed"#,
        payload.title
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                detail: "Failed to create todo".to_string(),
            }),
        )
    })?;

    Ok(Json(todo))
}

pub async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, impl IntoResponse> {
    match sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(AppError(
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                detail: "Todo not found".to_string(),
            }),
        )),
    }
}

pub async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    payload: Result<Json<UpdateTodo>, JsonRejection>,
) -> Result<Json<Todo>, impl IntoResponse> {
    let Json(payload) = payload?;

    let current_todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await;

    let todo = match current_todo {
        Ok(todo) => todo,
        Err(_) => {
            return Err(AppError(
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    detail: "Todo not found".to_string(),
                }),
            ))
        }
    };

    let title = payload.title.unwrap_or(todo.title);
    let completed = payload.completed.unwrap_or(todo.completed);

    match sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos 
        SET title = ?, completed = ?
        WHERE id = ?
        RETURNING id, title, completed
        "#,
        title,
        completed,
        id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(updated_todo) => Ok(Json(updated_todo)),
        Err(_) => Err(AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                detail: "Failed to update todo".to_string(),
            }),
        )),
    }
}

pub async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    match sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(AppError(
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        detail: "Todo not found".to_string(),
                    }),
                ))
            } else {
                Ok(StatusCode::NO_CONTENT)
            }
        }
        Err(_) => Err(AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                detail: "Failed to delete todo".to_string(),
            }),
        )),
    }
}
