use async_graphql::BatchRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphQLRequest {
  pub operations: BatchRequest,
  pub cancel_token: Option<String>,
}
