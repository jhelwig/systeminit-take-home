#![warn(clippy::all, clippy::pedantic)]

mod api;
mod router;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "systeminit_take_home=debug,tower_http=debug",
        );
    }
    tracing_subscriber::fmt::init();
    tracing::debug!("Logging initialized.");

    let app = router::build();

    let bind_address = match std::env::var("BIND_ADDRESS") {
        Ok(b) => b,
        Err(_) => "127.0.0.1".into(),
    };
    let bind_port = match std::env::var("BIND_PORT") {
        Ok(p) => p,
        Err(_) => "8000".into(),
    };
    let bind_string = format!("{}:{}", bind_address, bind_port);

    axum::Server::bind(&bind_string.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
