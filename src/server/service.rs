//! Registration services.

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::ServerState;

// Public
#[derive(Deserialize, Serialize)]
pub struct ReportInfo {
    name: String,
    uri: String,
}

pub async fn http_report(
    State(state): State<Arc<ServerState>>,
    Json(report_info): Json<ReportInfo>,
) -> StatusCode {
    log::info!("received from {}", report_info.name);

    let mut client_v = state.client_v.lock().await;
    client_v.insert(report_info.name.clone(), report_info.uri);
    StatusCode::OK
}

pub async fn http_list(State(state): State<Arc<ServerState>>) -> (StatusCode, String) {
    let client_v = state.client_v.lock().await;
    let mut list = Vec::new();
    for (name, uri) in &*client_v {
        list.push(ReportInfo {
            name: name.to_string(),
            uri: uri.clone(),
        })
    }
    match serde_json::to_string(&list) {
        Ok(r) => (StatusCode::OK, r),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn http_get_uri(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    let client_v = state.client_v.lock().await;
    match params.get("name") {
        Some(name) => match client_v.get(name) {
            Some(uri) => (StatusCode::OK, uri.clone()),
            None => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
        },
        None => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
    }
}
