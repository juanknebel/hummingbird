use axum::{routing::get, Router};
use dotenv;
use env_logger;
use log::info;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  let router = Router::new().route("/hello", get(hello));
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  info!("LISTENING on {addr}");
  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}

async fn hello() -> &'static str {
  "Hello hummingbird"
}
