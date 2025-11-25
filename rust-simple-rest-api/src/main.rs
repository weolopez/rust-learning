use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// Data model
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateItem {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateItem {
    name: Option<String>,
    completed: Option<bool>,
}

// App state
type Db = Arc<RwLock<HashMap<u64, Item>>>;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize state
    let db = Db::default();

    // Build our application with a route
    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
        .with_state(db);

    // Run our application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handlers
async fn list_items(State(db): State<Db>) -> Json<Vec<Item>> {
    let items = db.read().unwrap();
    Json(items.values().cloned().collect())
}

async fn create_item(
    State(db): State<Db>,
    Json(payload): Json<CreateItem>,
) -> impl IntoResponse {
    let mut items = db.write().unwrap();
    let id = items.keys().max().unwrap_or(&0) + 1;
    let item = Item {
        id,
        name: payload.name,
        completed: false,
    };
    items.insert(id, item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_item(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let items = db.read().unwrap();
    if let Some(item) = items.get(&id) {
        Ok(Json(item.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn update_item(
    Path(id): Path<u64>,
    State(db): State<Db>,
    Json(payload): Json<UpdateItem>,
) -> impl IntoResponse {
    let mut items = db.write().unwrap();
    if let Some(item) = items.get_mut(&id) {
        if let Some(name) = payload.name {
            item.name = name;
        }
        if let Some(completed) = payload.completed {
            item.completed = completed;
        }
        Ok(Json(item.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_item(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let mut items = db.write().unwrap();
    if items.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
