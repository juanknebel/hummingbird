use super::provider_client::{Provider, ProviderTurso};
use crate::{
  api::metadata::Metadata,
  resolver::{
    ad::{Ad, AdMatcher, SimpleMatcher},
    error::{Error, Result},
  },
};
use log::{error, info};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AdResolver<ProvClient> {
  provider: ProvClient,
  provider_fallback: ProviderTurso,
  ads: Arc<RwLock<Vec<Ad>>>,
  matcher: SimpleMatcher,
}

impl<ProvClient: Provider> AdResolver<ProvClient> {
  pub fn new(provider: ProvClient) -> Self {
    AdResolver {
      provider,
      provider_fallback: ProviderTurso::new(
        "asd".to_string(),
        "asd".to_string(),
      ),
      ads: Arc::new(RwLock::new(Vec::new())),
      matcher: SimpleMatcher,
    }
  }

  pub async fn resolve(&self, metadata: Metadata) -> Result<Ad> {
    info!("[selecting an ad]");
    let ads_lock =
      self.ads.read().map_err(|_| Error::CannotAcquirerReadLock)?;
    Ok(self.matcher.select_the_best(&ads_lock, &metadata))
  }

  pub async fn update_ads(&self) -> Result<()> {
    let new_ads = match self.provider.get_ads().await {
      Ok(ads_from_main) => ads_from_main,
      Err(e) => {
        error!("[Cannot update from main provider {e}]");
        info!("[Trying fallback]");
        match self.provider_fallback.get_ads().await {
          Ok(ads_from_fallback) => ads_from_fallback,
          Err(e) => {
            error!("[Cannot update from fallback provider {e}]");
            return Err(Error::CannotUpdateAds);
          },
        }
      },
    };

    let mut ads_lock = self
      .ads
      .write()
      .map_err(|_| Error::CannotAcquirerWriteLock)?;
    ads_lock.clone_from(&new_ads);
    let size = ads_lock.len();
    info!("[Updating successfully with {size} ads]");
    Ok(())
  }
}
