use ad::Ad;
use axum::{routing::get, Json, Router};
use log::info;
use rand::Rng;
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
  let mut rng = rand::thread_rng();
  let ads = vec![
    Ad::new(rng.gen_range(1..i32::MAX)),
    Ad::new(rng.gen_range(1..i32::MAX)),
    Ad::new(rng.gen_range(1..i32::MAX)),
  ];
  info!("[Returning 3 ads]");
  Ok(Json(ads))
}
