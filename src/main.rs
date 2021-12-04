#![warn(clippy::all, clippy::pedantic)]

mod api;
mod router;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "systeminit_take_home=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();
    tracing::debug!("Logging initialized.");

    let app = router::build_router();

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
