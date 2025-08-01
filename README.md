# Task Tracker API (Rust + Axum + SQLite)

A simple backend REST API for creating, updating, retrieving, filtering, and deleting tasks.

This project is written in Rust using the [Axum](https://github.com/tokio-rs/axum) framework and backed by a [SQLite](https://www.sqlite.org/) database using [SQLx](https://docs.rs/sqlx/latest/sqlx/).

---

## 🧠 Project Overview

This API allows clients to:

- Create new tasks
- Fetch all tasks (with optional filtering and pagination)
- Fetch a single task by ID
- Update task title or completion status
- Delete tasks
- Get structured error responses
- Store and retrieve data from a persistent SQLite database

---

## ⚙ How It Works

- **Axum** handles routing and async request handling
- **SQLx** communicates with a local `SQLite` database
- **Chrono** provides timestamp functionality
- **Serde** handles serialization/deserialization of JSON
- **UUID** is used to generate unique task IDs
- **In-memory storage** was used initially, but later replaced with a real database

The code is modularized into folders like `routes`, `models`, and `state` to separate concerns.

---

## 🚀 Running the Project

### 1. ✅ Prerequisites

You need:

- [Rust](https://www.rust-lang.org/tools/install)
- SQLite 3
- `sqlx-cli` installed:

```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

---

### 2. 📦 Add Required Environment Variable

You must set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL=sqlite://tasks.db
```

Alternatively, use a `.env` file and a crate like `dotenvy` if desired.

---

### 3. 🗃️ Run Database Migration

```bash
sqlx migrate run
```

If you've just cloned the repo and there are no migrations yet:

```bash
sqlx migrate add create_tasks
```

Then in the generated migration file (under `migrations/`), define the schema like this:

```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL,
    created_at TEXT NOT NULL
);
```

Then run the migration:

```bash
sqlx migrate run
```

---

### 4. 🧱 Build the Project

```bash
cargo build
```

---

### 5. 🚴 Run the API Server

```bash
cargo run
```

The server will start and listen on `http://localhost:3000`.

---

## 🛠 API Endpoints

| Method | Endpoint           | Description                      |
|--------|--------------------|----------------------------------|
| GET    | `/tasks`           | List all tasks (with filter/pagination) |
| POST   | `/tasks`           | Create a new task                |
| GET    | `/tasks/:id`       | Get task by ID                   |
| PATCH  | `/tasks/:id`       | Update a task                    |
| DELETE | `/tasks/:id`       | Delete a task                    |

### ✅ Example `curl` commands

```bash
# Create a task
curl -X POST http://localhost:3000/tasks      -H "Content-Type: application/json"      -d '{"title": "Do laundry"}'

# List all tasks
curl http://localhost:3000/tasks

# Filter completed tasks
curl "http://localhost:3000/tasks?completed=true"

# Get a task by ID
curl http://localhost:3000/tasks/<TASK_ID>

# Update a task
curl -X PATCH http://localhost:3000/tasks/<TASK_ID>      -H "Content-Type: application/json"      -d '{"completed": true}'

# Delete a task
curl -X DELETE http://localhost:3000/tasks/<TASK_ID>
```

---

## 📦 Dependencies

Here’s a full list of crates used:

```toml
[dependencies]
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite", "uuid", "chrono", "macros"] }
thiserror = "1"
```

If your app includes folders like `routes`, `models`, and `state`, they will be imported via `mod` declarations.

---

## 🙌 Final Notes

This is a backend-focused Rust project designed for portfolio-building. You could:

- Add CLI support
- Add a frontend later
- Deploy it with Docker
- Replace SQLite with Postgres or MySQL
- Add user authentication or auth tokens

Want to take it further? You already have a solid base.

---
