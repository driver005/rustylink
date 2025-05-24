use crate::SdkError;
use anyhow::Result;
use std::{fs, io::Write, path::Path, process::Command};

pub struct Config<'a> {
	pub(crate) graphql_url: Option<&'a str>,
	pub(crate) proto_url: Option<&'a str>,
	pub(crate) output_dir: &'a str,
}

impl<'a> Config<'a> {
	pub fn new() -> Self {
		Self {
			graphql_url: None,
			proto_url: None,
			output_dir: "src/test",
		}
	}
	pub fn graphql_url(mut self, url: &'a str) -> Self {
		self.graphql_url = Some(url);
		self
	}

	pub fn proto_url(mut self, url: &'a str) -> Self {
		self.proto_url = Some(url);
		self
	}

	pub fn output_dir(mut self, dir: &'a str) -> Self {
		self.output_dir = dir;
		self
	}
}

pub struct OutputFile<'a> {
	pub name: &'a str,
	pub content: String,
	pub dir: &'a str,
}

impl<'a> OutputFile<'a> {
	pub(crate) fn new(name: &'a str, content: String, dir: &'a str) -> Self {
		Self {
			name,
			content: format!("{}\n", content),
			dir,
		}
	}

	pub(crate) fn create(&self) -> Result<(), SdkError> {
		let save_dir = Path::new(self.dir);
		fs::create_dir_all(save_dir)?;

		let file_path = save_dir.join(self.name);
		println!("Writing {}", file_path.display());
		let mut file = fs::File::create(file_path)?;
		file.write_all(self.content.as_bytes())?;

		self.fmt_file()?;

		Ok(())
	}

	pub(crate) fn get_content(&self) -> &str {
		&self.content
	}

	pub(crate) fn add_content(&mut self, content: &str) {
		self.content.push_str(format!("{}\n", content).as_str());
	}

	pub(crate) fn fmt_file(&self) -> Result<()> {
		let file_path = Path::new(self.dir).join(self.name);
		let output = Command::new("rustfmt")
			.arg("--edition=2018")
			.arg("--config=normalize_doc_attributes=true")
			.arg(file_path)
			.spawn()
			.expect("rustfmt failed");
		output.wait_with_output()?;
		Ok(())
	}
}
