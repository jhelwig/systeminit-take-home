use axum::{
    AddExtensionLayer,
    extract::{
        ContentLengthLimit,
        Extension,
    },
    http::StatusCode,
    response::Json,
    Router,
    routing::{
        get,
        post,
    },
};
use serde_json::{
    json,
    Value,
};
use std::sync::{
    Arc,
    RwLock,
};

#[derive(Debug)]
struct AppState {
    messages: RwLock<Vec<String>>,
}

pub fn build() -> Router {
    let messages = RwLock::new(vec![]);
    let app_state = Arc::new(AppState { messages });

    get_router(app_state)
}

fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/messages", get(list_messages))
        .route("/messages", post(new_message))
        .layer(AddExtensionLayer::new(app_state))
}

async fn list_messages(Extension(app_state): Extension<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    let messages = match app_state.messages.read() {
        Ok(m) => m.clone(),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(json!(messages)))
}

async fn new_message(
    ContentLengthLimit(message): ContentLengthLimit<String, {1024 * 1_000 }>,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<Json<Value>, StatusCode> {
    match app_state.messages.write() {
        Ok(mut m) => m.push(message),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    list_messages(Extension(app_state)).await
}
