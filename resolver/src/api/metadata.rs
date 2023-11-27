use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Metadata {
  location: String,
  age: u8,
  gender: String,
}
