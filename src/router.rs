use crate::api;

use axum::{
    http::StatusCode,
    response::Redirect,
    Router,
    routing::get,
    routing::get_service,
};
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};

pub fn build() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/ui/index.html".parse().unwrap()) })
        )
        .nest(
            "/api",
            api::build()
        )
        .nest(
            "/ui",
            get_service(ServeDir::new("./ui/dist")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            })
        ).layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests{
    use super::*;
    use axum::{
        body::Body,
        http::{
            self,
            header,
            Request,
        },
    };
    use std::{
        fs::File,
        io::Write,
    };
    use tempfile::tempdir;
    use tower::ServiceExt;

    #[tokio::test]
    async fn serve_built_ui() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir.path()).unwrap();

        let ui_build_dir = temp_dir.path().join("ui/dist");
        std::fs::create_dir_all(&ui_build_dir).unwrap();
        let mut file = File::create(ui_build_dir.join("random_file.txt")).unwrap();
        file.write_all(b"Test content!").unwrap();

        let response = build()
            .oneshot(
                Request::get("/ui/random_file.txt")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"Test content!");
    }
}
