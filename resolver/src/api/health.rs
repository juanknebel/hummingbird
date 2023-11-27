use crate::error::Result;
use axum::{routing::get, Json, Router};
use log::info;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct Pong {
  message: String,
}

pub fn routes() -> Router {
  info!("[Adding GET /ping]");
  info!("[Adding GET /check]");
  Router::new()
    .route("/ping", get(pong))
    .route("/check", get(check))
}

async fn check() -> Result<Json<Pong>> {
  Ok(Json(Pong {
    message: "ok".to_string(),
  }))
}
async fn pong() -> &'static str {
  "pong"
}
