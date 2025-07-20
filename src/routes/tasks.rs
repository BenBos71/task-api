use axum::{
    extract::{Path, State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::{get},
};
use crate::models::task::{Task, CreateTask, PatchTask, TaskFilter, Pagination};
use crate::state::AppState;
use crate::errors::AppError;
use chrono::Utc;
use uuid::Uuid;

pub fn task_routes(state: AppState) -> Router {
    Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).patch(patch_task).delete(delete_task))
        .with_state(state)
}

async fn get_tasks(
    State(state): State<AppState>,
    Query(filter): Query<TaskFilter>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let mut query = String::from(
        "SELECT id, title, completed, created_at FROM tasks"
    );

    if let Some(completed) = filter.completed {
        query.push_str(" WHERE completed = ");
        query.push_str(if completed { "1" } else { "0" });
    }

    query.push_str(" ORDER BY created_at DESC");

    if let Some(limit) = pagination.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }
    if let Some(offset) = pagination.offset {
        query.push_str(&format!(" OFFSET {}", offset));
    }

    let tasks = sqlx::query_as::<_, Task>(&query)
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(tasks))
}

async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, title, completed, created_at
        FROM tasks
        WHERE id = ?
        "#,
        task_id.to_string()
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    match task {
        Some(task) => Ok(Json(task)),
        None => Err(AppError::NotFound("Task not found".into())),
    }
}

async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query!(
        "DELETE FROM tasks WHERE id = ?",
        task_id.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound("Task not found".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".into()));
    }

    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        completed: false,
        created_at: Utc::now(),
    };

    sqlx::query!(
        r#"
        INSERT INTO tasks (id, title, completed, created_at)
        VALUES (?, ?, ?, ?)
        "#,
        task.id.to_string(),
        task.title,
        task.completed,
        task.created_at
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(task)))
}

async fn patch_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<PatchTask>,
) -> Result<impl IntoResponse, AppError> {
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, title, completed, created_at
        FROM tasks
        WHERE id = ?
        "#,
        task_id.to_string()
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let Some(mut task) = task else {
        return Err(AppError::NotFound("Task not found".into()));
    };

    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".into()));
        }
        task.title = title;
    }

    if let Some(completed) = payload.completed {
        task.completed = completed;
    }

    sqlx::query!(
        r#"
        UPDATE tasks SET title = ?, completed = ? WHERE id = ?
        "#,
        task.title,
        task.completed,
        task.id.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(task))
}