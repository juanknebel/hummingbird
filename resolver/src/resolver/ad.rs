use crate::api::metadata::Metadata;
use serde::Serialize;
use std::time::Duration;

#[derive(Clone, Debug, Default, Serialize)]
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
