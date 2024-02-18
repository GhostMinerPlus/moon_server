//! Server that provides registration services.

use std::{io, sync::Arc, time::Duration};

use axum::{routing, Router};
use tokio::time;

use crate::state;

mod service;

async fn serve(server: &Server) -> io::Result<()> {
    // build our application with a route
    let app = Router::new()
        .route(
            &format!("/{}/report", server.name),
            routing::post(service::http_report),
        )
        .route(
            &format!("/{}/list", server.name),
            routing::get(service::http_list),
        )
        .route(
            &format!("/{}/get", server.name),
            routing::get(service::http_get_uri),
        )
        .with_state(server.state.clone());

    // run our app with hyper, listening globally on port 3000
    let address = format!("{}:{}", server.ip, server.port);
    log::info!("serving at {address}");
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// Public
pub struct Server {
    ip: String,
    port: u16,
    name: String,
    state: Arc<state::ServerState>,
}

impl Server {
    pub fn new(ip: String, port: u16, name: String) -> Self {
        Self {
            ip,
            port,
            name,
            state: Arc::new(state::ServerState::default()),
        }
    }

    pub async fn run(self) -> io::Result<()> {
        let state = self.state.clone();
        tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_secs(60)).await;
                let mut client_v = state.client_v.lock().await;
                client_v.clear();
            }
        });
        serve(&self).await
    }
}
