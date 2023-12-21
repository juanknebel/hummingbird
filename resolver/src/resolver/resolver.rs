use super::provider_client::{Provider, ProviderTurso};
use crate::{
  api::metadata::Metadata,
  resolver::{
    ad_matcher::AdMatcher,
    error::{Error, Result},
  },
};
use ad::Ad;
use log::{error, info};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AdResolver<Matcher, ProvClient> {
  provider: ProvClient,
  provider_fallback: ProviderTurso,
  ads: Arc<RwLock<Vec<Ad>>>,
  matcher: Matcher,
}

impl<Matcher: AdMatcher, ProvClient: Provider> AdResolver<Matcher, ProvClient> {
  pub fn new(matcher: Matcher, provider: ProvClient) -> Self {
    AdResolver {
      provider,
      provider_fallback: ProviderTurso::new(
        "asd".to_string(),
        "asd".to_string(),
      ),
      ads: Arc::new(RwLock::new(Vec::new())),
      matcher
    }
  }

  pub async fn resolve(&self, metadata: Metadata) -> Result<Ad> {
    info!("[selecting an ad]");
    let ads_lock =
      self.ads.read().map_err(|_| Error::CannotAcquirerReadLock)?;
    Ok(self.matcher.select_the_best(&ads_lock, &metadata))
  }

  pub async fn update_ads(&self) -> Result<()> {
    let new_ads = self.retrieve_ads().await?;

    let mut ads_lock = self
      .ads
      .write()
      .map_err(|_| Error::CannotAcquirerWriteLock)?;
    ads_lock.clone_from(&new_ads);
    let size = ads_lock.len();
    info!("[Updating successfully with {size} ads]");
    Ok(())
  }

  async fn retrieve_ads(&self) -> Result<Vec<Ad>> {
    match self.provider.get_ads().await {
      Ok(ads) => Ok(ads),
      Err(err) => {
        error!("[Cannot update from main provider {err}]");
        info!("[Trying fallback]");
        match self.provider_fallback.get_ads().await {
          Ok(ads) => Ok(ads),
          Err(other_err) => {
            error!("[Cannot update from fallback provider {other_err}]");
            Err(Error::CannotUpdateAds)
          }
        }
      }
    }
  }
}
