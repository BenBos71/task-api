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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let database_url = "sqlite://tasks.db";
    let state = AppState::new(database_url).await?;

    let app = task_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// curl http://localhost:3000/tasks