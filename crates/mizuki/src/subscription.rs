use async_graphql::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubscriptionRequest {
  #[serde(flatten)]
  pub inner: Request,
  pub id: u32,
  pub sub_id: String,
}
