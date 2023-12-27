use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::util::AppState;

#[derive(Deserialize, Serialize)]
pub struct ReportInfo {
    name: String,
    ipv6: String,
}

pub async fn http_report(
    State(state): State<Arc<AppState>>,
    Json(report_info): Json<ReportInfo>,
) -> StatusCode {
    log::info!("received from {}", report_info.name);

    let mut client_v = state.client_v.lock().await;
    client_v.insert(report_info.name, report_info.ipv6);
    StatusCode::OK
}

pub async fn http_list(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    let client_v = state.client_v.lock().await;
    let mut list = Vec::new();
    for (name, ipv6) in &*client_v {
        list.push(ReportInfo {
            name: name.to_string(),
            ipv6: ipv6.to_string(),
        })
    }
    match serde_json::to_string(&list) {
        Ok(r) => (StatusCode::OK, r),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
