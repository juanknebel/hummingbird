use crate::{
  api::metadata::Metadata,
  error::{Error, Result},
  resolver::{provider_client::ProviderHttp, resolver::AdResolver},
};
use ad::Ad;
use axum::{
  extract::{rejection::JsonRejection, State},
  routing::post,
  Json, Router,
};
use log::info;
use std::sync::Arc;

pub fn routes(resolver: Arc<AdResolver<ProviderHttp>>) -> Router {
  info!("[Adding POST /resolve]");
  Router::new()
    .route("/resolve", post(resolve))
    .with_state(resolver)
}

async fn resolve(
  State(ad_resolver): State<Arc<AdResolver<ProviderHttp>>>,
  payload: core::result::Result<Json<Metadata>, JsonRejection>,
) -> Result<Json<Ad>> {
  let Json(metadata) = payload.map_err(|e| Error::ParseError {
    kind: e.body_text(),
  })?;
  match ad_resolver.resolve(metadata).await {
    Ok(ad) => Ok(Json(ad)),
    Err(_) => return Err(Error::NotFound),
  }
}
