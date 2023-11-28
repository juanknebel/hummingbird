use axum::{routing::get, Json, Router};
use log::info;
use rand::Rng;
use serde::Serialize;
use std::{net::SocketAddr, str::FromStr, time::Duration};

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

#[derive(Debug, Clone, Serialize)]
pub struct Ad {
  id: i32,
  gender: String,
  city: String,
  state: String,
  country: String,
  language: String,
  incomes: String,
  price: f32,
  duration: Duration,
}
impl Ad {
  pub fn new(id: i32) -> Self {
    Ad {
      id,
      gender: "BOTH".to_string(),
      city: "CITY".to_string(),
      state: "STATE".to_string(),
      country: "COUNTRY".to_string(),
      language: "EN".to_string(),
      incomes: "MIDDLE".to_string(),
      price: 1.0,
      duration: Duration::from_secs(15u64),
    }
  }
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
