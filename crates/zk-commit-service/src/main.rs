use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use zk_commit_service::{build_app, controllers::*, AppState};

pub fn init_logger() {
    // let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    use env_logger::Builder;
    // Initialize the logger builder
    Builder::new()
        // Set the minimum log level to display
        .filter_level(log::LevelFilter::Info)
        // Set the format for the log output
        .format_timestamp_secs() // Disable seconds
        .format_timestamp_millis() // Enable milliseconds
        // Initialize the logger
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let app_state = web::Data::new(AppState::new());
    let server = HttpServer::new(move || {
        let app = build_app!(gateway, app_state);
        app
    });

    let _ = server.bind(("127.0.0.1", 8080))?.run().await;

    Ok(())
}
