pub type Result<T> = core::result::Result<T, Error>;

/// The resolver module errors.
#[derive(Clone, Debug, strum_macros::AsRefStr)]
pub enum Error {
  ProviderConnectionFail,
  NoAds,
  CannotUpdateAds,
  CannotAcquirerWriteLock,
  CannotAcquirerReadLock,
}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}
