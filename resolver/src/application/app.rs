use crate::{
  api::{api, health},
  resolver::{provider_client::ProviderHttp, resolver::AdResolver},
};
use axum::Router;
use dotenv::dotenv;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

pub struct Application {
  addr: SocketAddr,
  routes: Vec<Router>,
}

impl Application {
  pub fn new() -> Application {
    dotenv().ok();
    // -- Configurations
    let host = std::env::var("HOST").expect("Host undefined");
    let provider_host =
      std::env::var("PROVIDER_URL").expect("Provider host must be defined");
    let update_every =
      std::env::var("UPDATE_EVERY").map_or("30".to_string(), |s| s);
    let update_every =
      Duration::from_secs(u64::from_str(update_every.as_str()).unwrap());

    let provider = ProviderHttp::new(provider_host);
    let resolver = AdResolver::new(provider);

    // -- Routes
    let mut routes = vec![];
    routes.push(api::routes(Arc::new(resolver.clone())));
    routes.push(health::routes());

    // -- Scheduled tasks
    tokio::spawn(updating_ads(update_every, resolver));

    Application {
      addr: SocketAddr::from_str(&host).unwrap(),
      routes,
    }
  }

  pub fn address(&self) -> SocketAddr {
    self.addr.clone()
  }

  pub fn routers(&self) -> Vec<Router> {
    self.routes.clone()
  }
}

async fn updating_ads(duration: Duration, resolver: AdResolver<ProviderHttp>) {
  loop {
    resolver.update_ads().await.ok();
    tokio::time::sleep(duration).await;
  }
}
