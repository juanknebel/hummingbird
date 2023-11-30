use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
