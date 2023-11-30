use crate::api::metadata::Metadata;
use ad::Ad;

/// The trait that specifies how an ad matcher behaviour
pub trait AdMatcher {
  /// Based on an ad and the metadata, select the best suitable ad for the
  /// metadata.
  /// # Arguments:
  /// * ads: a vector with all the available ads.
  /// * metadata: the metadata.
  /// # Return:
  /// * an Ad which is the more suitable.
  fn select_the_best(&self, ads: &[Ad], metadata: &Metadata) -> Ad;
}

/// A very simple matcher.
#[derive(Clone, Debug)]
pub struct SimpleMatcher;

impl SimpleMatcher {
  fn is_a_match(&self, an_ad: &Ad, metadata: &Metadata) -> bool {
    true
  }
}

impl AdMatcher for SimpleMatcher {
  fn select_the_best(&self, ads: &[Ad], metadata: &Metadata) -> Ad {
    let matching_ads: Vec<_> = ads
      .iter()
      .cloned()
      .filter(|an_ad| self.is_a_match(&an_ad, metadata))
      .collect();
    matching_ads.get(0).unwrap().clone()
  }
}
