use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use log::error;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

/// The application errors.
/// As the application grows this must be split in specific errors per module.
/// But at the moment, is better to centralize the definition.
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
  LoginFail,
  // -- Auth errors.
  AuthFailNoAuthTokenCookie,
  AuthFailTokenWrongFormat,
  AuthFailCtxNotInRequestExt,
  ParseError { kind: String },
  NotFound,
}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let (status_code, client_error) = self.client_status_and_error();
    error!("[client_error: {client_error}] [trace: {self:?}]");
    let mut response = status_code.into_response();

    // Insert the Error into the response.
    response.extensions_mut().insert(self);

    response
  }
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    #[allow(unreachable_patterns)]
    match self {
      Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

      // -- Auth.
      Self::AuthFailNoAuthTokenCookie
      | Self::AuthFailTokenWrongFormat
      | Self::AuthFailCtxNotInRequestExt => {
        (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
      },
      // -- Parse
      Self::ParseError {
        ..
      } => (
        StatusCode::BAD_REQUEST,
        ClientError::INVALID_PARAMS,
      ),
      // -- NOt found
      Self::NotFound => (
        StatusCode::NOT_FOUND,
        ClientError::NOT_MATCHING_AD,
      ),
      // -- Fallback.
      _ => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
    }
  }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
  LOGIN_FAIL,
  NO_AUTH,
  INVALID_PARAMS,
  SERVICE_ERROR,
  NOT_MATCHING_AD,
}

impl core::fmt::Display for ClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}
