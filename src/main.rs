use axum::{
    routing::{get, delete, patch},
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;
use uuid::Uuid;
use std::{sync::{Arc, Mutex}, collections::HashMap};

#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
struct PatchTask {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Clone)]
struct AppState {
    tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    completed: bool,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info")
        .init();

    let state = AppState {
        tasks: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", patch(patch_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
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

async fn get_tasks(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let tasks = state.tasks.lock().map_err(|_| AppError::Internal("Lock poisoned".into()))?;

    let task_list: Vec<Task> = tasks.values().cloned().collect();

    Ok(Json(task_list))
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

// curl http://localhost:3000/health
// curl http://localhost:3000/tasks