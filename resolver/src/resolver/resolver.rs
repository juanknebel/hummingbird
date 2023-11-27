use super::provider_client::Provider;
use crate::{
  api::metadata::Metadata,
  resolver::{
    ad::{Ad, AdMatcher, SimpleMatcher},
    error::{Error, Result},
  },
};
use log::info;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AdResolver<ProvClient> {
  provider: ProvClient,
  ads: Arc<RwLock<Vec<Ad>>>,
  matcher: SimpleMatcher,
}

impl<ProvClient: Provider> AdResolver<ProvClient> {
  pub fn new(provider: ProvClient) -> Self {
    AdResolver {
      provider,
      ads: Arc::new(RwLock::new(Vec::new())),
      matcher: SimpleMatcher,
    }
  }

  pub async fn resolve(&self, metadata: Metadata) -> Result<Ad> {
    info!("[selecting an ad]");
    let ads_lock =
      self.ads.read().map_err(|_| Error::CannotAcquirerReadLock)?;
    let matching_ads: Vec<_> = ads_lock
      .iter()
      .cloned()
      .filter(|an_ad| self.matcher.is_a_match(&an_ad, &metadata))
      .collect();
    Ok(self.matcher.select_the_best(&matching_ads))
  }

  pub async fn update_ads(&self) -> Result<()> {
    let new_ads = self.provider.get_ads().await?;
    let mut ads_lock = self
      .ads
      .write()
      .map_err(|_| Error::CannotAcquirerWriteLock)?;
    ads_lock.clone_from(&new_ads);
    info!("[Updating successfully]");
    Ok(())
  }
}
