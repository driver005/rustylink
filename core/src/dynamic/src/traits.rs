use crate::{SeaResult, Value};
use ordered_float::OrderedFloat;
use std::{
	any::Any,
	borrow::Cow,
	collections::{BTreeMap, HashMap},
	fmt::Display,
};

pub trait ValueAccessorTrait<'a>: Sized {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ObjectAccessor: ObjectAccessorTrait<'a>;
	type ListAccessor: ListAccessorTrait<'a>;

	fn type_name(&self) -> &'static str;
	fn is_null(&self) -> bool;
	fn as_value(&self) -> Self::Value;

	fn int8(&self) -> Result<i8, Self::Error>;
	fn int16(&self) -> Result<i16, Self::Error>;
	fn int32(&self) -> Result<i32, Self::Error>;
	fn int64(&self) -> Result<i64, Self::Error>;
	fn int128(&self) -> Result<i128, Self::Error>;
	fn intsize(&self) -> Result<isize, Self::Error>;
	fn uint8(&self) -> Result<u8, Self::Error>;
	fn uint16(&self) -> Result<u16, Self::Error>;
	fn uint32(&self) -> Result<u32, Self::Error>;
	fn uint64(&self) -> Result<u64, Self::Error>;
	fn uint128(&self) -> Result<u128, Self::Error>;
	fn uintsize(&self) -> Result<usize, Self::Error>;
	fn float32(&self) -> Result<OrderedFloat<f32>, Self::Error>;
	fn float64(&self) -> Result<OrderedFloat<f64>, Self::Error>;
	fn bool(&self) -> Result<bool, Self::Error>;
	fn char(&self) -> Result<char, Self::Error>;
	fn string(&self) -> Result<&str, Self::Error>;
	fn object(&self) -> Result<Self::ObjectAccessor, Self::Error>;
	fn list(&self) -> Result<Self::ListAccessor, Self::Error>;
	fn option(&self) -> Result<Option<Self>, Self::Error>;
	fn variable(&self) -> Result<(Self, Self), Self::Error>;
	fn enum_name(&self) -> Result<&str, Self::Error>;

	// fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Self::Error>;
	// fn upload(&self) -> Result<Upload, Self::Error>;
}

pub trait ObjectAccessorTrait<'a> {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor: ValueAccessorTrait<'a>;

	fn type_name(&self) -> &'static str;
	// fn get_accessor(self) -> ObjectAccessors<'a>;

	fn get<T: Into<Self::Value>>(&'a self, name: T) -> Option<Self::ValueAccessor>;
	fn try_get<T: Into<Self::Value>>(&'a self, name: T)
	-> Result<Self::ValueAccessor, Self::Error>;
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = (&'a Self::Value, Self::ValueAccessor)> + 'a>;
	fn keys(&'a self) -> Box<dyn Iterator<Item = &'a Self::Value> + 'a>;
	fn values(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a>;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn as_index_map(&self) -> BTreeMap<Self::Value, Self::Value>;
}

pub trait ListAccessorTrait<'a>: Sized {
	type Value;
	type Error: std::fmt::Debug + Send + Sync + 'static;
	type ValueAccessor: ValueAccessorTrait<'a>;

	fn type_name(&self) -> &'static str;
	// fn get_accessor<'b>(&'b self) -> ListAccessors<'b>;

	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn to_iter(&'a self) -> Box<dyn Iterator<Item = Self::ValueAccessor> + 'a>;
	fn get(&'a self, idx: usize) -> Option<Self::ValueAccessor>;
	fn try_get(&'a self, idx: usize) -> Result<Self::ValueAccessor, Self::Error>;
	fn as_slice(&self, start: usize, end: usize) -> Result<Self, Self::Error>;
	fn as_values_slice(&self) -> Vec<Self::Value>;
}

pub trait EnumTrait {
	type Item: EnumItemTrait;

	fn new(name: impl Into<String>) -> Self;

	fn item(self, item: impl Into<Self::Item>) -> Self;

	fn items(self, fields: impl IntoIterator<Item = impl Into<Self::Item>>) -> Self;

	fn type_name(&self) -> &str;
}

pub trait EnumItemTrait {
	fn new(name: impl Into<String>, tag: u32) -> Self;

	fn type_name(&self) -> &str;
}

/// Trait for common behaviors of TypeRef
pub trait TypeRefTrait: Sized {
	fn named(type_name: impl Into<String>) -> Self;
	fn named_nn(type_name: impl Into<String>) -> Self;
	fn named_list(type_name: impl Into<String>) -> Self;
	fn named_nn_list(type_name: impl Into<String>) -> Self;
	fn named_list_nn(type_name: impl Into<String>) -> Self;
	fn named_nn_list_nn(type_name: impl Into<String>) -> Self;
	fn non_null(ty: Box<Self>) -> Self;
	fn type_name(&self) -> &str;

	const DOUBLE: &'static str;
	const FLOAT: &'static str;
	const INT32: &'static str;
	const INT64: &'static str;
	const UINT32: &'static str;
	const UINT64: &'static str;
	const SINT32: &'static str;
	const SINT64: &'static str;
	const FIXED32: &'static str;
	const FIXED64: &'static str;
	const SFIXED32: &'static str;
	const SFIXED64: &'static str;
	const BOOL: &'static str;
	const STRING: &'static str;
	const BYTES: &'static str;
	const ID: &'static str;
	// const UPLOAD: &'static str;
}

pub trait FieldValueTrait<'a>: Sized + Send + Sync {
	type Value;
	type Error;

	const NULL: Self;
	const NONE: Option<Self>;

	fn null() -> Self;
	fn none() -> Option<Self>;
	fn value(value: impl Into<Self::Value>) -> Self;
	fn owned_any<T: Any + Send + Sync>(obj: T) -> Self;
	fn boxed_any(obj: Box<dyn Any + Send + Sync>) -> Self;
	fn borrowed_any(obj: &'a (dyn Any + Send + Sync)) -> Self;
	fn list<I, T>(values: I) -> Self
	where
		I: IntoIterator<Item = T>,
		T: Into<Self>;
	fn with_type(self, ty: impl Into<Cow<'static, str>>) -> Self;
	fn as_value(&self) -> Option<&Self::Value>;
	fn try_to_value(&self) -> Result<&Self::Value, Self::Error>;
	fn as_list(&'a self) -> Option<&'a [Self]>;
	fn try_to_list(&'a self) -> Result<&'a [Self], Self::Error>;
	fn downcast_ref<T: Any>(&self) -> Option<&T>;
	fn try_downcast_ref<T: Any>(&self) -> Result<&T, Self::Error>;
	fn to_val(&self) -> Option<Self::Value>;
}

pub trait ResolverContextDyn<'a> {
	type Context;
	type ObjectAccessor: ObjectAccessorTrait<'a>;
	type FieldValue: FieldValueTrait<'a>;

	fn ctx(&'a self) -> &'a Self::Context;
	fn args(self) -> Self::ObjectAccessor;
	fn parent_value(&'a self) -> &'a Self::FieldValue;
}

pub trait FieldFutureTrait<'a> {
	type Error;
	type ValueType;
	type FieldValue: FieldValueTrait<'a, Error = Self::Error, Value = Self::ValueType>;
	/// Create a `FieldFuture` from a `Future`
	fn new<Fut, R>(future: Fut) -> Self
	where
		Fut: Future<Output = Result<Option<R>, Self::Error>> + Send + 'a,
		R: Into<Self::FieldValue> + Send;

	/// Create a `FieldFuture` from a `Value`
	fn from_value(value: Option<Self::ValueType>) -> Self;
}

pub trait ErrorTrait {
	fn new(message: impl Into<String>) -> Self;

	/// Implement `From<T>` for each type as needed
	fn to<T>(value: T) -> Self
	where
		T: Display + Send + Sync + 'static;
}

pub trait ValueTrait<'a>: Send + Sync {
	type FieldValue: FieldValueTrait<'a>;

	fn new<T>(val: T) -> Self
	where
		T: From<T>;

	fn into_field_value(self) -> Self::FieldValue;
}

// impl<T: Display + Send + Sync + 'static, E: ErrorTrait> From<T> for E {
// 	fn from(value: T) -> Self {
// 		todo!()
// 	}
// }

/// Represents a GraphQL input type.
pub trait InputType: Send + Sync + Sized {
	/// The raw type used for validator.
	///
	/// Usually it is `Self`, but the wrapper type is its internal type.
	///
	/// For example:
	///
	/// `i32::RawValueType` is `i32`
	/// `Option<i32>::RawValueType` is `i32`.
	type RawValueType: ?Sized;

	/// Type the name.
	fn type_name() -> Cow<'static, str>;

	/// Qualified typename.
	fn qualified_type_name() -> String {
		format!("{}!", Self::type_name())
	}

	// /// Create type information in the registry and return qualified typename.
	// fn create_type_info(registry: &mut GraphQLRegistry) -> String;

	/// Parse from `Value`. None represents undefined.
	fn parse(value: Option<Value>) -> SeaResult<Self>;

	/// Convert to a `Value` for introspection.
	fn to_value(&self) -> Value;

	/// Get the federation fields, only for InputObject.
	#[doc(hidden)]
	fn federation_fields() -> Option<String> {
		None
	}

	/// Returns a reference to the raw value.
	fn as_raw_value(&self) -> Option<&Self::RawValueType>;
}

pub trait BatchFn<K, V> {
	fn load(&self, keys: &[K]) -> impl std::future::Future<Output = HashMap<K, V>>;
}
