use dynamic::prelude::{
	GraphQLError, GraphQLField, GraphQLFieldFuture, GraphQLObject, GraphQLValue, ProtoError,
	ProtoField, ProtoFieldFuture, ProtoMessage, ProtoValue,
};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use sea_orm::{ColumnTrait, ColumnType, EntityName, EntityTrait, IdenStatic, Iterable, ModelTrait};
use std::ops::Add;
/// The configuration structure for EntityObjectBuilder
pub struct EntityObjectConfig {
	/// used to format the type name of the object
	pub type_name: crate::SimpleNamingFn,
	/// used to format the name of column fields
	pub column_name: crate::ComplexNamingFn,
	/// suffix that is appended on basic version of entity type
	pub basic_type_suffix: String,
}

impl std::default::Default for EntityObjectConfig {
	fn default() -> Self {
		Self {
			type_name: Box::new(|entity_name: &str| -> String {
				entity_name.to_upper_camel_case()
			}),
			column_name: Box::new(|_entity_name: &str, column_name: &str| -> String {
				column_name.to_lower_camel_case()
			}),
			basic_type_suffix: "Basic".into(),
		}
	}
}

use crate::{BuilderContext, GuardAction, TypesMapHelper};

/// This builder produces the GraphQL object of a SeaORM entity
pub struct EntityObjectBuilder {
	pub context: &'static BuilderContext,
}

impl EntityObjectBuilder {
	/// used to get type name
	pub fn type_name<T>(&self) -> String
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let name: String = <T as EntityName>::table_name(&T::default()).into();
		self.context.entity_object.type_name.as_ref()(&name)
	}

	/// used to get type name for basic version
	pub fn basic_type_name<T>(&self) -> String
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let name: String = <T as EntityName>::table_name(&T::default()).into();
		format!(
			"{}{}",
			self.context.entity_object.type_name.as_ref()(&name),
			self.context.entity_object.basic_type_suffix
		)
	}

	/// used to get column field name of entity column
	pub fn column_name<T>(&self, column: &T::Column) -> String
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_name = self.type_name::<T>();
		let column_name: String = column.as_str().into();
		self.context.entity_object.column_name.as_ref()(&entity_name, &column_name)
	}

	/// used to get the GraphQL object of a SeaORM entity
	pub fn to_object<T>(&self) -> GraphQLObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let object_name = self.type_name::<T>();

		self.basic_object::<T>(&object_name)
	}

	/// used to get the GraphQL basic object of a SeaORM entity
	pub fn basic_to_object<T>(&self) -> GraphQLObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let object_name = self.basic_type_name::<T>();

		self.basic_object::<T>(&object_name)
	}

	/// used to create a SeaORM entity basic GraphQL object type
	fn basic_object<T>(&self, object_name: &str) -> GraphQLObject
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_name = self.type_name::<T>();

		let types_map_helper = TypesMapHelper {
			context: self.context,
		};

		T::Column::iter().fold(GraphQLObject::new(object_name), |object, column: T::Column| {
			let column_name = self.column_name::<T>(&column);

			// println!("object_name: {} column_name: {}", object_name, column_name);

			let column_def = column.def();

			let graphql_type = match types_map_helper.sea_orm_column_type_to_graphql_type(
				column_def.get_column_type(),
				!column_def.is_null(),
			) {
				Some(type_name) => type_name,
				None => return object,
			};

			// This isn't the most beautiful flag: it's indicating whether the leaf type is an
			// enum, rather than the type itself. Ideally we'd only calculate this for the leaf
			// type itself. Could be a good candidate for refactor as this code evolves to support
			// more container types. For example, this at the very least should be recursive on
			// Array types such that arrays of arrays of enums would be resolved correctly.
			let is_enum: bool = match column_def.get_column_type() {
				ColumnType::Enum {
					..
				} => true,
				#[cfg(feature = "with-postgres-array")]
				ColumnType::Array(inner) => matches!(inner.as_ref(), ColumnType::Enum { .. }),
				_ => false,
			};

			let guard = self
				.context
				.guards_graphql
				.field_guards
				.get(&format!("{}.{}", &object_name, &column_name));

			let conversion_fn = self
				.context
				.types
				.output_conversions_graphql
				.get(&format!("{entity_name}.{column_name}"));

			let field = GraphQLField::new(column_name, graphql_type, move |ctx| {
				let guard_flag = if let Some(guard) = guard {
					(*guard)(&ctx)
				} else {
					GuardAction::Allow
				};

				if let GuardAction::Block(reason) = guard_flag {
					return GraphQLFieldFuture::new(async move {
						match reason {
							Some(reason) => {
								Err::<Option<()>, GraphQLError>(GraphQLError::new(reason))
							}
							None => Err::<Option<()>, GraphQLError>(GraphQLError::new(
								"GraphQLField guard triggered.",
							)),
						}
					});
				}

				// convert SeaQL value to GraphQL value
				// FIXME: move to types_map file
				let object = ctx
					.parent_value
					.try_downcast_ref::<T::Model>()
					.expect("Something went wrong when trying to downcast entity object.");

				if let Some(conversion_fn) = conversion_fn {
					let result = conversion_fn(&object.get(column));
					return GraphQLFieldFuture::new(async move {
						match result {
							Ok(value) => Ok(Some(value)),
							// FIXME: proper error reporting
							Err(_) => Ok(None),
						}
					});
				}

				GraphQLFieldFuture::new(async move {
					Ok(sea_query_value_to_graphql_value(object.get(column), is_enum))
				})
			});

			object.field(field)
		})
	}

	/// used to get the Proto message of a SeaORM entity
	pub fn to_message<T>(&self) -> ProtoMessage
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let object_name = self.type_name::<T>();

		self.basic_message::<T>(&object_name)
	}

	/// used to get the Proto basic message of a SeaORM entity
	pub fn basic_to_message<T>(&self) -> ProtoMessage
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let object_name = self.basic_type_name::<T>();

		self.basic_message::<T>(&object_name)
	}

	/// used to create a SeaORM entity basic Proto message type
	fn basic_message<T>(&self, object_name: &str) -> ProtoMessage
	where
		T: EntityTrait,
		<T as EntityTrait>::Model: Sync,
	{
		let entity_name = self.type_name::<T>();

		let types_map_helper = TypesMapHelper {
			context: self.context,
		};

		T::Column::iter().enumerate().fold(
			ProtoMessage::new(object_name),
			|object, (index, column)| {
				let column_name = self.column_name::<T>(&column);

				let column_def = column.def();

				let proto_type = match types_map_helper.sea_orm_column_type_to_proto_type(
					column_def.get_column_type(),
					!column_def.is_null(),
				) {
					Some(type_name) => type_name,
					None => return object,
				};

				// This isn't the most beautiful flag: it's indicating whether the leaf type is an
				// enum, rather than the type itself. Ideally we'd only calculate this for the leaf
				// type itself. Could be a good candidate for refactor as this code evolves to support
				// more container types. For example, this at the very least should be recursive on
				// Array types such that arrays of arrays of enums would be resolved correctly.
				let is_enum: bool = match column_def.get_column_type() {
					ColumnType::Enum {
						..
					} => true,
					#[cfg(feature = "with-postgres-array")]
					ColumnType::Array(inner) => matches!(inner.as_ref(), ColumnType::Enum { .. }),
					_ => false,
				};

				let guard = self
					.context
					.guards_proto
					.field_guards
					.get(&format!("{}.{}", &object_name, &column_name));

				let conversion_fn = self
					.context
					.types
					.output_conversions_proto
					.get(&format!("{entity_name}.{column_name}"));

				let field =
					ProtoField::output(column_name, index.add(1) as u32, proto_type, move |ctx| {
						let guard_flag = if let Some(guard) = guard {
							(*guard)(&ctx)
						} else {
							GuardAction::Allow
						};

						if let GuardAction::Block(reason) = guard_flag {
							return ProtoFieldFuture::new(async move {
								match reason {
									Some(reason) => {
										Err::<Option<()>, ProtoError>(ProtoError::new(reason))
									}
									None => Err::<Option<()>, ProtoError>(ProtoError::new(
										"ProtoField guard triggered.",
									)),
								}
							});
						}

						// convert SeaQL value to GraphQL value
						// FIXME: move to types_map file
						let object = ctx
							.parent_value
							.try_downcast_ref::<T::Model>()
							.expect("Something went wrong when trying to downcast entity object.");

						if let Some(conversion_fn) = conversion_fn {
							let result = conversion_fn(&object.get(column));
							return ProtoFieldFuture::new(async move {
								match result {
									Ok(value) => Ok(Some(value)),
									// FIXME: proper error reporting
									Err(_) => Ok(None),
								}
							});
						}

						ProtoFieldFuture::new(async move {
							Ok(sea_query_value_to_proto_value(object.get(column), is_enum))
						})
					});

				object.field(field)
			},
		)
	}
}

fn sea_query_value_to_graphql_value(
	sea_query_value: sea_orm::sea_query::Value,
	is_enum: bool,
) -> Option<GraphQLValue> {
	match sea_query_value {
		sea_orm::Value::Bool(value) => value.map(GraphQLValue::from),
		sea_orm::Value::TinyInt(value) => value.map(GraphQLValue::from),
		sea_orm::Value::SmallInt(value) => value.map(GraphQLValue::from),
		sea_orm::Value::Int(value) => value.map(GraphQLValue::from),
		sea_orm::Value::BigInt(value) => value.map(GraphQLValue::from),
		sea_orm::Value::TinyUnsigned(value) => value.map(GraphQLValue::from),
		sea_orm::Value::SmallUnsigned(value) => value.map(GraphQLValue::from),
		sea_orm::Value::Unsigned(value) => value.map(GraphQLValue::from),
		sea_orm::Value::BigUnsigned(value) => value.map(GraphQLValue::from),
		sea_orm::Value::Float(value) => value.map(GraphQLValue::from),
		sea_orm::Value::Double(value) => value.map(GraphQLValue::from),
		sea_orm::Value::String(value) if is_enum => value
			.map(|it| GraphQLValue::from(it.as_str().to_upper_camel_case().to_ascii_uppercase())),
		sea_orm::Value::String(value) => value.map(|it| GraphQLValue::from(it.as_str())),
		sea_orm::Value::Char(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[allow(clippy::box_collection)]
		sea_orm::Value::Bytes(value) => value.map(|it| GraphQLValue::from(String::from_utf8_lossy(&it))),

		#[cfg(feature = "with-postgres-array")]
		sea_orm::Value::Array(_array_value, value) => value.map(|it| {
			GraphQLValue::List(
				it.into_iter()
					.map(|item| {
						sea_query_value_to_graphql_value(item, is_enum)
							.unwrap_or(GraphQLValue::Null)
					})
					.collect(),
			)
		}),

		#[cfg(feature = "with-json")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
		sea_orm::sea_query::Value::Json(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDate(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoTime(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTime(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeUtc(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeLocal(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeWithTimeZone(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDate(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeTime(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDateTime(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDateTimeWithTimeZone(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		#[cfg(feature = "with-uuid")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
		sea_orm::sea_query::Value::Uuid(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[cfg(feature = "with-decimal")]
		sea_orm::sea_query::Value::Decimal(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		#[cfg(feature = "with-bigdecimal")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
		sea_orm::sea_query::Value::BigDecimal(value) => {
			value.map(|it| GraphQLValue::from(it.to_string()))
		}

		// #[cfg(feature = "with-ipnetwork")]
		// #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
		// sea_orm::sea_query::Value::IpNetwork(value) => value.map(|it| GraphQLValue::from(it.to_string())),

		// #[cfg(feature = "with-mac_address")]
		// #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
		// sea_orm::sea_query::Value::MacAddress(value) => value.map(|it| GraphQLValue::from(it.to_string())),
		#[allow(unreachable_patterns)]
		_ => panic!("Cannot convert SeaORM value"),
	}
}

fn sea_query_value_to_proto_value(
	sea_query_value: sea_orm::sea_query::Value,
	is_enum: bool,
) -> Option<ProtoValue> {
	println!("sea_query_value_to_proto_value: {}", sea_query_value);
	match sea_query_value {
		sea_orm::Value::Bool(value) => value.map(ProtoValue::from),
		sea_orm::Value::TinyInt(value) => value.map(ProtoValue::from),
		sea_orm::Value::SmallInt(value) => value.map(ProtoValue::from),
		sea_orm::Value::Int(value) => value.map(ProtoValue::from),
		sea_orm::Value::BigInt(value) => value.map(ProtoValue::from),
		sea_orm::Value::TinyUnsigned(value) => value.map(ProtoValue::from),
		sea_orm::Value::SmallUnsigned(value) => value.map(ProtoValue::from),
		sea_orm::Value::Unsigned(value) => value.map(ProtoValue::from),
		sea_orm::Value::BigUnsigned(value) => value.map(ProtoValue::from),
		sea_orm::Value::Float(value) => value.map(ProtoValue::from),
		sea_orm::Value::Double(value) => value.map(ProtoValue::from),
		sea_orm::Value::String(value) if is_enum => {
			value.map(|it| ProtoValue::from(it.as_str().to_upper_camel_case().to_ascii_uppercase()))
		}
		sea_orm::Value::String(value) => value.map(|it| ProtoValue::from(it.as_str())),
		sea_orm::Value::Char(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[allow(clippy::box_collection)]
		sea_orm::Value::Bytes(value) => value.map(|it| ProtoValue::from(String::from_utf8_lossy(&it))),

		#[cfg(feature = "with-postgres-array")]
		sea_orm::Value::Array(_array_value, value) => value.map(|it| {
			ProtoValue::List(
				it.into_iter()
					.map(|item| {
						sea_query_value_to_graphql_value(item, is_enum).unwrap_or(ProtoValue::Null)
					})
					.collect(),
			)
		}),

		#[cfg(feature = "with-json")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
		sea_orm::sea_query::Value::Json(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDate(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoTime(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTime(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeUtc(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeLocal(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-chrono")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
		sea_orm::sea_query::Value::ChronoDateTimeWithTimeZone(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDate(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeTime(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDateTime(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-time")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
		sea_orm::sea_query::Value::TimeDateTimeWithTimeZone(value) => {
			value.map(|it| ProtoValue::from(it.to_string()))
		}

		#[cfg(feature = "with-uuid")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
		sea_orm::sea_query::Value::Uuid(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-decimal")]
		sea_orm::sea_query::Value::Decimal(value) => value.map(|it| ProtoValue::from(it.to_string())),

		#[cfg(feature = "with-bigdecimal")]
		#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
		sea_orm::sea_query::Value::BigDecimal(value) => value.map(|it| ProtoValue::from(it.to_string())),

		// #[cfg(feature = "with-ipnetwork")]
		// #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
		// sea_orm::sea_query::Value::IpNetwork(value) => value.map(|it| ProtoValue::from(it.to_string())),

		// #[cfg(feature = "with-mac_address")]
		// #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
		// sea_orm::sea_query::Value::MacAddress(value) => value.map(|it| ProtoValue::from(it.to_string())),
		#[allow(unreachable_patterns)]
		_ => panic!("Cannot convert SeaORM value"),
	}
}
