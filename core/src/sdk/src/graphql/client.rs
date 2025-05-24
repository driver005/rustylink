use reqwest::{
	header::{HeaderMap, HeaderName, HeaderValue},
	Client,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{graphql::GraphQLErrorMessage, SdkError};

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
	data: Option<T>,
	errors: Option<Vec<GraphQLErrorMessage>>,
}

#[derive(Debug, Clone)]
pub struct GraphQLClient<'a> {
	client: Client,
	url: &'a str,
	header_map: HeaderMap,
}

impl<'a> GraphQLClient<'a> {
	pub fn new(url: &'a str) -> Self {
		let mut header_map = HeaderMap::new();

		header_map.insert(
			HeaderName::from_static("content-type"),
			HeaderValue::from_static("application/json"),
		);

		Self {
			client: Client::new(),
			url,
			header_map,
		}
	}

	pub async fn request<K>(&self, query: &str, function_name: &str) -> Result<K, SdkError>
	where
		K: for<'de> Deserialize<'de> + std::fmt::Debug,
	{
		// Create the request body
		let request_body = serde_json::json!({
			"query": query,
		});

		// Send the request
		let response = self
			.client
			.post(self.url)
			.headers(self.header_map.clone()) // If authentication is needed
			.json(&request_body)
			.send()
			.await?;

		// Deserialize the response JSON
		let json_response = response.json::<GraphQLResponse<Value>>().await;

		// Check whether JSON is parsed successfully
		match json_response {
			Ok(json) => {
				// Check if error messages have been received
				if let Some(err) = json.errors {
					return Err(SdkError::GraphQlError(err));
				}

				if let Some(val) = json.data.unwrap().get(function_name) {
					Ok(serde_json::from_value(val.to_owned())?)
				} else {
					Err(SdkError::Custom(format!("{} not found in response", function_name)))
				}
			}
			Err(e) => Err(SdkError::HttpError(e)),
		}
	}
}
