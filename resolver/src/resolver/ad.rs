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

pub trait AdMatcher {
  fn is_a_match(&self, an_ad: &Ad, metadata: &Metadata) -> bool;
  fn select_the_best(&self, ads: &Vec<Ad>) -> Ad;
}

#[derive(Clone, Debug)]
pub struct SimpleMatcher;

impl AdMatcher for SimpleMatcher {
  fn is_a_match(&self, an_ad: &Ad, metadata: &Metadata) -> bool {
    true
  }

  fn select_the_best(&self, ads: &Vec<Ad>) -> Ad {
    ads.get(0).unwrap().clone()
  }
}
