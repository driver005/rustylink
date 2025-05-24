use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
	#[error("The GraphQL server returned an error")]
	GraphQlError(Vec<crate::graphql::GraphQLErrorMessage>),
	#[error(transparent)]
	CynicError(#[from] cynic::http::CynicReqwestError),
	#[error("Couldn't parse a header from {0}.  Make sure you've passed a header of the form `Name: Value`")]
	MalformedHeaderArgument(String),
	#[error("Couldn't convert introspection result into schema: {0}")]
	SchemaError(cynic_introspection::SchemaError),
	#[error(
		"The introspection query seems to have failed.  Try looking in the response for errors"
	)]
	IntrospectionQueryFailed,
	#[error("Couldn't write the schema to file: {0}")]
	IOError(#[from] std::io::Error),
	#[error("Couldn't codegen from schemafile: {0}")]
	SchemaRegistration(#[from] cynic_codegen::registration::SchemaRegistrationError),
	#[error("Couldn't write the schema to file: {0}")]
	IntrospectionQuery(#[from] anyhow::Error),
	#[error(transparent)]
	HttpError(#[from] reqwest::Error),
	#[error("{0}")]
	Custom(String),
	#[error("Couldn't not serialize values: {0}")]
	Serialize(#[from] serde_json::Error),
}

impl From<cynic_introspection::SchemaError> for SdkError {
	fn from(value: cynic_introspection::SchemaError) -> Self {
		match value {
			cynic_introspection::SchemaError::IntrospectionQueryFailed => {
				SdkError::IntrospectionQueryFailed
			}
			other => SdkError::SchemaError(other),
		}
	}
}

pub type Result<T> = std::result::Result<T, SdkError>;
