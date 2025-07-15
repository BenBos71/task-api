mod models;
mod routes;
mod errors;
mod state;

// use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber;
use crate::state::AppState;
use crate::routes::tasks::task_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info")
        .init();

    let state = AppState::new(); // helper constructor we'll add

    let app = task_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// curl http://localhost:3000/health
// curl http://localhost:3000/tasks