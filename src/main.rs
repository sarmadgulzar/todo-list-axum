use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{routing::get, serve, Router};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::net::TcpListener;

const DATABASE_URL: &str = "sqlite://db.sqlite3";

#[derive(Serialize, sqlx::FromRow)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Serialize)]
struct ErrorResponse {
    detail: String,
}

struct AppError(StatusCode, Json<ErrorResponse>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(err: JsonRejection) -> Self {
        AppError(
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                detail: err.to_string(),
            }),
        )
    }
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        Sqlite::create_database(DATABASE_URL).await.unwrap();
    }

    let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(pool);

    let listner = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listner, app).await.unwrap();
}

async fn list_todos(State(pool): State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(Todo, "SELECT id, title, completed FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(todos)
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    // Validate title is not empty
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
        r#"
    INSERT INTO todos (title, completed) VALUES (?1, false) RETURNING id, title, completed"#,
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

async fn get_todo(
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

async fn update_todo(
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

async fn delete_todo(
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
