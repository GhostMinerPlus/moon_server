mod service;
// mod task;
mod util;

use axum::{routing, Router};
use serde::Deserialize;
use std::{
    fs,
    io::{self, Error},
    sync::Arc,
};

#[derive(Deserialize, Clone)]
struct Config {
    address: String,
    name: String,
    thread_num: u8,
}

fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("INFO")).init();
    let config_s = fs::read_to_string("config.toml")?;
    let config: Config =
        toml::from_str(&config_s).map_err(|e| Error::new(io::ErrorKind::Other, e.message()))?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.thread_num as usize)
        .build()?
        .block_on(async move {
            start_task(&config).await?;
            serve(&config).await
        })?;
    Ok(())
}

async fn start_task(_: &Config) -> io::Result<()> {
    Ok(())
}

async fn serve(config: &Config) -> io::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route(
            &format!("/{}/report", config.name),
            routing::post(service::http_report),
        )
        .route(
            &format!("/{}/list", config.name),
            routing::get(service::http_list),
        )
        .with_state(Arc::new(util::AppState::default()));

    // run our app with hyper, listening globally on port 3000
    log::info!("serving at {}", config.address);
    let listener = tokio::net::TcpListener::bind(&config.address).await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
