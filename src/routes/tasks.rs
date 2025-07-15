use axum::{
    extract::{Path, State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::{get},
};
use crate::models::task::{Task, CreateTask, PatchTask, TaskFilter};
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
) -> Result<impl IntoResponse, AppError> {
    let tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;

    let filtered_tasks: Vec<Task> = tasks
        .values()
        .cloned()
        .filter(|task| {
            match filter.completed {
                Some(wanted) => task.completed == wanted,
                None => true, // no filter applied
            }
        })
        .collect();

    Ok(Json(filtered_tasks))
}

async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;

    match tasks.get(&task_id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(AppError::NotFound("Task not found".into())),
    }
}

async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let mut tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;

    if tasks.remove(&task_id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound("Task not found".into()))
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

    let mut tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;
    tasks.insert(task.id, task.clone());

    Ok((StatusCode::CREATED, Json(task)))
}

async fn patch_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<PatchTask>,
) -> Result<impl IntoResponse, AppError> {
    let mut tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;

    let task = tasks.get_mut(&task_id).ok_or_else(|| AppError::NotFound("Task not found".into()))?;

    if let Some(title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".into()));
        }
        task.title = title;
    }

    if let Some(completed) = payload.completed {
        task.completed = completed;
    }

    Ok(Json(task.clone()))
}