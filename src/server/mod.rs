//! Server that provides registration services.

use std::{io, sync::Arc};

use axum::{routing, Router};

use crate::state;

mod service;

// Public
pub struct Server {
    ip: String,
    port: u16,
    name: String,
}

impl Server {
    pub fn new(ip: String, port: u16, name: String) -> Self {
        Self { ip, port, name }
    }

    pub async fn run(self) -> io::Result<()> {
        // build our application with a route
        let app = Router::new()
            .route(
                &format!("/{}/report", self.name),
                routing::post(service::http_report),
            )
            .route(
                &format!("/{}/list", self.name),
                routing::get(service::http_list),
            )
            .route(
                &format!("/{}/get", self.name),
                routing::get(service::http_get_address),
            )
            .with_state(Arc::new(state::ServerState::default()));

        // run our app with hyper, listening globally on port 3000
        let address = format!("{}:{}", self.ip, self.port);
        log::info!("serving at {address}");
        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
