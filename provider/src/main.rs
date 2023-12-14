use ad::Ad;
use axum::{routing::get, Json, Router};
use log::info;
use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  let router = Router::new().route("/provide/ads", get(list_ads));
  let host =
    std::env::var("PROVIDER_HOST").expect("Provider host must be specified");
  let addr = SocketAddr::from_str(host.as_str()).unwrap();
  info!("[Listening on {addr}]");
  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}

async fn list_ads() -> Result<Json<Vec<Ad>>, String> {
  let ads = vec![Ad::default(), Ad::default(), Ad::default()];
  info!("[Returning 3 ads]");
  Ok(Json(ads))
}
