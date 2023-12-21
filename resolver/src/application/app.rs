use crate::{
  api::{api, health},
  resolver::{provider_client::ProviderHttp, resolver::AdResolver},
};
use axum::Router;
use dotenv::dotenv;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use crate::resolver::ad_matcher::{AdMatcher, SimpleMatcher};
use crate::resolver::provider_client::Provider;

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
    let matcher = SimpleMatcher;
    let resolver: AdResolver<SimpleMatcher, ProviderHttp> = AdResolver::new(matcher, provider);

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

/// Scheduled task in charge of updating the ads the resolver could return.
/// For now its implementation is using an infinite loop.
/// # Arguments
/// * duration: the interval in which it going to perform the action.
/// * resolver: the resolver who is going to execute the action.
async fn updating_ads(duration: Duration, resolver: AdResolver<impl AdMatcher, impl Provider>) {
  loop {
    resolver.update_ads().await.ok();
    tokio::time::sleep(duration).await;
  }
}
