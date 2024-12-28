// use protobuf::descriptor::field_descriptor_proto::Type;
// use protobuf::reflect::FieldDescriptor;
// use protobuf::reflect::MessageDescriptor;
// use protobuf_codegen::Customize;
// use protobuf_codegen::CustomizeCallback;
use proto_builder_trait::tonic::BuilderAttributes;
use std::error::Error;
use std::path::PathBuf;
// struct GenDerives;

// impl CustomizeCallback for GenDerives {
// 	fn message(&self, _message: &MessageDescriptor) -> Customize {
// 		Customize::default()
// 			.before("#[derive(serde::Serialize, serde::Deserialize, sea_orm::FromJsonQueryResult)]")
// 	}

// 	fn enumeration(&self, _enum_type: &protobuf::reflect::EnumDescriptor) -> Customize {
// 		Customize::default().before("#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]\n#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
// 	}

// 	fn field(&self, field: &FieldDescriptor) -> Customize {
// 		if field.proto().type_() == Type::TYPE_ENUM {
// 			// `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
// 			Customize::default().before(
//                     "#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")]")
// 		} else {
// 			Customize::default()
// 		}
// 	}

// 	fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
// 		Customize::default().before("#[serde(skip)]\n#[sea_orm(skip)]")
// 	}
// }

fn main() -> Result<(), Box<dyn Error>> {
	let out_dir = PathBuf::from("./src/admin");

	let _ = std::fs::create_dir(out_dir.clone());

	// // Use this in build.rs
	// protobuf_codegen::Codegen::new()
	// 	// Use `protoc` parser, optional.
	// 	.protoc()
	// 	// Use `protoc-bin-vendored` bundled protoc command, optional.
	// 	.protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
	// 	// All inputs and imports from the inputs must reside in `includes` directories.
	// 	.includes(&["../.."])
	// 	// Inputs must reside in some of include paths.
	// 	.inputs(vec![
	// 		"../../protobuf/link/column.proto",
	// 		"../../protobuf/link/core.proto",
	// 		"../../protobuf/link/extension.proto",
	// 		"../../protobuf/link/foreigntable.proto",
	// 		"../../protobuf/link/function.proto",
	// 		"../../protobuf/link/index.proto",
	// 		"../../protobuf/link/polices.proto",
	// 		"../../protobuf/link/publication.proto",
	// 		"../../protobuf/link/role.proto",
	// 		"../../protobuf/link/schema.proto",
	// 		"../../protobuf/link/table.proto",
	// 		"../../protobuf/link/trigger.proto",
	// 		"../../protobuf/link/types.proto",
	// 		"../../protobuf/link/view.proto",
	// 	])
	// 	// .customize_callback(GenDerives)
	// 	// Specify output directory relative to Cargo output directory.
	// 	.out_dir(out_dir)
	// 	.run()?;

	// tonic_build::configure()
	// 	.file_descriptor_set_path(out_dir.join("column_descriptor.bin"))
	// 	.compile(&["../../protobuf/link/column.proto"], &["proto"])?;

	tonic_build::configure()
		.out_dir(out_dir.clone())
		.build_client(false)
		.build_server(true)
		.build_transport(false)
		// .message_attribute(".", "#[derive(sea_orm::FromJsonQueryResult)]")
		// .enum_attribute(".", "#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]\n#[sea_orm(rs_type = \"i32\", db_type = \"Integer\")]")
		// .with_serde(&["."], true, true, None)
		.compile(
			&[
				// "../../protobuf/link/column.proto",
				// "../../protobuf/link/core.proto",
				// "../../protobuf/link/extension.proto",
				// "../../protobuf/link/foreigntable.proto",
				// "../../protobuf/link/function.proto",
				// "../../protobuf/link/index.proto",
				// "../../protobuf/link/polices.proto",
				// "../../protobuf/link/publication.proto",
				// "../../protobuf/link/role.proto",
				// "../../protobuf/link/schema.proto",
				// "../../protobuf/link/table.proto",
				// "../../protobuf/link/trigger.proto",
				// "../../protobuf/link/types.proto",
				// "../../protobuf/link/view.proto",
				"./src/plugin/proto/user.proto",
			],
			&["./"],
		)?;
	Ok(())
}
