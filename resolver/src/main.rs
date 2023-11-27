mod api;
mod application;
mod error;
mod resolver;

use crate::application::app::Application;
use axum::Router;
use dotenv;
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  let mut router = Router::new();
  let application = Application::new();
  let addr = application.address();
  for a_route in application.routers() {
    router = router.merge(a_route);
  }
  info!("[Listening on {addr}]");
  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}
