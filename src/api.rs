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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{
            self,
            Request,
        },
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn get_no_messages() {
        let app_state = Arc::new(AppState { messages: RwLock::new(vec![]) });
        let router = get_router(app_state);

        let response = router
            .oneshot(Request::get("/messages").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let expected_body: Vec<String> = vec![];
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(expected_body));
    }

    #[tokio::test]
    async fn get_some_messages() {
        let app_state = Arc::new(
        AppState {
                messages: RwLock::new(vec![
                    "Message 1".into(),
                    "Message 2".into()
                ])
            }
        );
        let router = get_router(app_state);

        let response = router
            .oneshot(Request::get("/messages").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let expected_body: Vec<String> = vec![
            "Message 1".into(),
            "Message 2".into(),
        ];
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(expected_body));
    }

    #[tokio::test]
    async fn add_new_message() {
        let app_state = Arc::new(AppState {messages: RwLock::new(vec![])});
        let router = get_router(app_state.clone());

        let body_content = "The new message";
        let response = router
            .oneshot(
                Request::post("/messages")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::CONTENT_LENGTH, body_content.bytes().count())
                    .body(Body::from(body_content))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let expected_messages: Vec<String> = vec![
            "The new message".into(),
        ];
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(expected_messages));

        assert_eq!(
            app_state.messages.read().unwrap().clone(),
            expected_messages,
        );
    }

    #[tokio::test]
    async fn add_new_message_with_existing() {
        let app_state = Arc::new(AppState {messages: RwLock::new(vec!["Existing message".into()])});
        let router = get_router(app_state.clone());

        let body_content = "The new message";
        let response = router
            .oneshot(
                Request::post("/messages")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::CONTENT_LENGTH, body_content.bytes().count())
                    .body(Body::from(body_content))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let expected_messages: Vec<String> = vec![
            "Existing message".into(),
            "The new message".into(),
        ];
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(expected_messages));

        assert_eq!(
            app_state.messages.read().unwrap().clone(),
            expected_messages,
        );
    }
}
