use cynic::http::ReqwestBlockingExt;
use cynic_introspection::{IntrospectionQuery, SpecificationVersion};
use reqwest::blocking::Client;

use crate::{
	graphql::{parse_schema, parse_schema_file, RendererConfig},
	Config, OutputFile, SdkError,
};

pub struct Builder {}

impl Builder {
	pub fn get_schema<'a>(dir: &'a str, url: &'a str) -> Result<OutputFile<'a>, SdkError> {
		// We can run an introspection query and unwrap the data contained within
		//TODO: replace run_graphql
		//  let response = self.json(&operation).send()?;

		//     let status = response.status();
		//     if !status.is_success() {
		//         let body_string = response.text().map_err(CynicReqwestError::ReqwestError)?;

		//         match serde_json::from_str::<GraphQlResponse<ResponseData>>(&body_string) {
		//             Ok(response) => return Ok(response),
		//             Err(_) => {
		//                 return Err(CynicReqwestError::ErrorResponse(status, body_string));
		//             }
		//         };
		//     }
		let response = Client::new().post(url).run_graphql(
			IntrospectionQuery::with_capabilities(SpecificationVersion::October2021.capabilities()),
		)?;

		let errors = response.errors.unwrap_or_default();
		if !errors.is_empty() {
			eprintln!("{}", "Errors while introspecting: ",);

			for error in errors {
				eprintln!("- {}", error.message);
			}
			eprintln!();
		}

		let Some(data) = response.data else {
			return Err(SdkError::IntrospectionQueryFailed);
		};

		let schema = data.into_schema()?;

		Ok(OutputFile::new("schema.graphql", schema.to_sdl(), dir))
	}

	pub fn build<'a>(config: Config<'a>) -> Result<(), SdkError> {
		let render_config = RendererConfig::default();
		let output_dir = format!("{}/graphql", config.output_dir);
		let backup_dir = format!("{}/backup", config.output_dir);

		let schema_result = if let Some(url) = config.graphql_url {
			let schema = Self::get_schema(&backup_dir, url)?;

			schema.create()?;

			parse_schema(schema.get_content(), &render_config)
		} else {
			parse_schema_file((backup_dir + "/schema.graphql").as_str(), &render_config)
		};

		match schema_result {
			Ok(structured_schema) => match structured_schema.output(render_config, &output_dir) {
				Ok(()) => {
					println!("files outputed in {}", output_dir);
				}

				Err(e) => {
					println!("{}", e);
				}
			},
			Err(e) => return Err(SdkError::IntrospectionQuery(e)),
		};

		Ok(())
	}
}
