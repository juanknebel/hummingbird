use crate::resolver::error::{Error, Result};
use ad::Ad;
use axum::async_trait;

#[async_trait]
pub trait Provider: Send + Sync + 'static {
  async fn get_ads(&self) -> Result<Vec<Ad>>;
}

#[derive(Clone)]
pub struct ProviderHttp {
  host: String,
}

impl ProviderHttp {
  pub fn new(host: String) -> Self {
    ProviderHttp {
      host,
    }
  }
}

#[async_trait]
impl Provider for ProviderHttp {
  async fn get_ads(&self) -> Result<Vec<Ad>> {
    let endpoint = self.host.to_string() + "/provide/ads";
    let response = reqwest::get(endpoint.as_str()).await.map_err(|e| {
      Error::ProviderConnectionFail {
        kind: e.to_string(),
      }
    })?;
    let ads = response
      .json::<Vec<Ad>>()
      .await
      .map_err(|_| Error::CannotParseAds)?;
    Ok(ads)
  }
}

#[derive(Clone)]
pub struct ProviderTurso {
  host: String,
  token: String,
}

impl ProviderTurso {
  pub fn new(host: String, token: String) -> Self {
    ProviderTurso {
      host,
      token,
    }
  }
}

#[async_trait]
impl Provider for ProviderTurso {
  async fn get_ads(&self) -> Result<Vec<Ad>> {
    Ok(vec![Ad::default(), Ad::default()])
  }
}
