use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ad {
  id: i32,
  user_target: UserTarget,
  location: Location,
  price: f32,
  duration: Duration,
  owner: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserTarget {
  gender: String,
  incomes: String,
  language: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Location {
  city: String,
  state: String,
  country: String,
}

impl Default for Ad {
  fn default() -> Self {
    let mut rng = rand::thread_rng();
    Ad {
      id: rng.gen_range(1..i32::MAX),
      user_target: UserTarget::default(),
      location: Location::default(),
      price: 1.0,
      duration: Duration::from_secs(15u64),
      owner: "".to_string(),
    }
  }
}
