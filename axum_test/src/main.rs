use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
}

#[tokio::main]
async fn main() {
    let mut user_map: HashMap<String, User> = HashMap::new();

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(create_user))
        .route("/query", post(query_user))
        .route("/del", post(del_user))
        .layer(Extension(Arc::new(RwLock::new(user_map))));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// which calls one of these handlers
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Extension(state): Extension<Arc<RwLock<HashMap<String, User>>>>,
    user: Json<User>,
) -> Json<User> {
    let mut data = state.write().unwrap();
    data.insert(user.name.clone(), user.0.clone());
    println!("Received user: {:?}", user.name);
    println!("{:#?}", data);
    user
}

async fn query_user(
    Extension(state): Extension<Arc<RwLock<HashMap<String, User>>>>,
    user: Json<User>,
) -> Json<Option<User>> {
    let data = state.read().unwrap();
    let res = data.get(&user.name).cloned();
    Json(res)
}

async fn del_user(
    Extension(state): Extension<Arc<RwLock<HashMap<String, User>>>>,
    user: Json<User>,
) -> String {
    let mut data = state.write().unwrap();
    let res = match data.remove(&user.name) {
        Some(_) => "ok",
        None => "不存在该用户",
    };
    res.to_string()
}
