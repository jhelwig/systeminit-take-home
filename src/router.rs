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

pub fn build_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/ui/index.html".parse().unwrap()) })
        )
        .nest(
            "/api",
            api::build_router()
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
