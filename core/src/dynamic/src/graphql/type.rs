use super::{
	Argument, Arguments, Enum, ExecutionResult, Executor, GraphQLType, GraphQLValueAsync,
	Interface, JuniperField, JuniperTypeRef, MetaType, Object, Registry, Scalar, Subscription,
	Union,
};
use crate::{BoxFieldFuture, ContextBase, FieldValue, ObjectAccessor, Value};
use futures::future::BoxFuture;
use lazy_static::lazy_static;
use std::{
	collections::BTreeMap,
	sync::{Arc, RwLock},
};

lazy_static! {
	pub(crate) static ref TYPES: Arc<RwLock<BTreeMap<String, Arc<Type>>>> =
		Arc::new(RwLock::new(BTreeMap::new()));
}

// Functions to interact with TYPES

pub fn add_type(name: String, ty: Type) {
	let mut types = TYPES.write().unwrap(); // Write access to the RwLock
	types.insert(name, Arc::new(ty));
}

pub fn get_type(name: &str) -> Option<Arc<Type>> {
	let types = TYPES.read().unwrap(); // Read access to the RwLock
	types.get(name).map(|ty| ty.clone())
}

/// A GraphQL type
#[derive(Debug)]
pub enum Type {
	/// Scalar
	Scalar(Scalar),
	/// Object
	Object(Object),
	/// Enum
	Enum(Enum),
	/// Interface
	Interface(Interface),
	/// Union
	Union(Union),
	/// Subscription
	Subscription(Subscription),
}

impl Type {
	pub(crate) fn name(&self) -> &str {
		match self {
			Type::Scalar(scalar) => scalar.type_name(),
			Type::Object(object) => object.type_name(),
			Type::Enum(e) => e.type_name(),
			Type::Interface(interface) => interface.type_name(),
			Type::Union(union) => union.type_name(),
			Type::Subscription(subscription) => subscription.type_name(),
		}
	}

	#[inline]
	pub(crate) fn as_object(&self) -> Option<&Object> {
		if let Type::Object(obj) = self {
			Some(obj)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn as_interface(&self) -> Option<&Interface> {
		if let Type::Interface(interface) = self {
			Some(interface)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn as_subscription(&self) -> Option<&Subscription> {
		if let Type::Subscription(subscription) = self {
			Some(subscription)
		} else {
			None
		}
	}

	pub(crate) fn is_output_type(&self) -> bool {
		match self {
			Type::Scalar(_) => true,
			Type::Object(_) => true,
			Type::Enum(_) => true,
			Type::Interface(_) => true,
			Type::Union(_) => true,
			Type::Subscription(_) => false,
		}
	}

	pub(crate) fn is_input_type(&self) -> bool {
		match self {
			Type::Scalar(_) => true,
			Type::Object(_) => true,
			Type::Enum(_) => true,
			Type::Interface(_) => false,
			Type::Union(_) => false,
			Type::Subscription(_) => false,
		}
	}

	pub(crate) fn field<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.field::<Scalar>(field_name, scalar),
			Type::Object(object) => registry.field::<Object>(field_name, object),
			Type::Enum(e) => registry.field::<Enum>(field_name, e),
			Type::Interface(interface) => registry.field::<Interface>(field_name, interface),
			Type::Union(union) => registry.field::<Union>(field_name, union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn field_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.field::<Option<Scalar>>(field_name, scalar),
			Type::Object(object) => registry.field::<Option<Object>>(field_name, object),
			Type::Enum(e) => registry.field::<Option<Enum>>(field_name, e),
			Type::Interface(interface) => {
				registry.field::<Option<Interface>>(field_name, interface)
			}
			Type::Union(union) => registry.field::<Option<Union>>(field_name, union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn field_vec<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.field::<Option<Scalar>>(field_name, scalar),
			Type::Object(object) => registry.field::<Option<Object>>(field_name, object),
			Type::Enum(e) => registry.field::<Option<Enum>>(field_name, e),
			Type::Interface(interface) => {
				registry.field::<Option<Interface>>(field_name, interface)
			}
			Type::Union(union) => registry.field::<Option<Union>>(field_name, union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn field_vec_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.field::<Option<Vec<Scalar>>>(field_name, scalar),
			Type::Object(object) => registry.field::<Option<Vec<Object>>>(field_name, object),
			Type::Enum(e) => registry.field::<Option<Vec<Enum>>>(field_name, e),
			Type::Interface(interface) => {
				registry.field::<Option<Vec<Interface>>>(field_name, interface)
			}
			Type::Union(union) => registry.field::<Option<Vec<Union>>>(field_name, union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn argument<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Scalar>(field_name, scalar),
			Type::Object(object) => registry.arg::<Object>(field_name, object),
			Type::Enum(e) => registry.arg::<Enum>(field_name, e),
			Type::Interface(interface) => unimplemented!(),
			Type::Union(union) => unimplemented!(),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn argument_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Option<Scalar>>(field_name, scalar),
			Type::Object(object) => registry.arg::<Option<Object>>(field_name, object),
			Type::Enum(e) => registry.arg::<Option<Enum>>(field_name, e),
			Type::Interface(interface) => unimplemented!(),
			Type::Union(union) => unimplemented!(),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn argument_vec<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Vec<Scalar>>(field_name, scalar),
			Type::Object(object) => registry.arg::<Vec<Object>>(field_name, object),
			Type::Enum(e) => registry.arg::<Vec<Enum>>(field_name, e),
			Type::Interface(interface) => unimplemented!(),
			Type::Union(union) => unimplemented!(),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn argument_vec_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Option<Vec<Scalar>>>(field_name, scalar),
			Type::Object(object) => registry.arg::<Option<Vec<Object>>>(field_name, object),
			Type::Enum(e) => registry.arg::<Option<Vec<Enum>>>(field_name, e),
			Type::Interface(interface) => unimplemented!(),
			Type::Union(union) => unimplemented!(),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn get_type<'a>(&self, registry: &mut Registry<'a, Value>) -> JuniperTypeRef<'a> {
		match self {
			Type::Scalar(scalar) => registry.get_type::<Scalar>(scalar),
			Type::Object(object) => registry.get_type::<Object>(object),
			Type::Enum(e) => registry.get_type::<Enum>(e),
			Type::Interface(interface) => registry.get_type::<Interface>(interface),
			Type::Union(union) => registry.get_type::<Union>(union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn meta<'a>(&self, registry: &mut Registry<'a, Value>) -> MetaType<'a, Value> {
		match self {
			Type::Scalar(scalar) => Scalar::meta(scalar, registry),
			Type::Object(object) => Object::meta(object, registry),
			Type::Enum(e) => Enum::meta(e, registry),
			Type::Interface(interface) => Interface::meta(interface, registry),
			Type::Union(union) => Union::meta(union, registry),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn collect<'a>(
		&'a self,
		ctx: &'a ContextBase,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFuture<'a>> {
		match self {
			Type::Scalar(scalar) => Vec::from([scalar.collect(arguments)]),
			Type::Object(object) => object.collect(ctx, arguments, parent_value),
			Type::Enum(en) => Vec::from([en.collect(arguments)]),
			Type::Interface(interface) => todo!(),
			Type::Union(union) => todo!(),
			Type::Subscription(subscription) => todo!(),
		}
	}

	pub fn resolve<'a>(
		&'a self,
		field: &'a str,
		arguments: &'a Arguments<Value>,
		executor: &'a Executor<ContextBase, Value>,
	) -> BoxFuture<'a, ExecutionResult<Value>> {
		match self {
			Type::Scalar(scalar) => scalar.resolve_field_async(scalar, field, arguments, executor),
			Type::Object(object) => object.resolve_field_async(object, field, arguments, executor),
			Type::Enum(e) => e.resolve_field_async(e, field, arguments, executor),
			Type::Interface(interface) => {
				interface.resolve_field_async(interface, field, arguments, executor)
			}
			Type::Union(union) => union.resolve_field_async(union, field, arguments, executor),
			Type::Subscription(subscription) => todo!(),
		}
	}
}

impl From<Scalar> for Type {
	#[inline]
	fn from(scalar: Scalar) -> Self {
		Type::Scalar(scalar)
	}
}

impl From<Object> for Type {
	#[inline]
	fn from(obj: Object) -> Self {
		Type::Object(obj)
	}
}

impl From<Enum> for Type {
	#[inline]
	fn from(e: Enum) -> Self {
		Type::Enum(e)
	}
}

impl From<Interface> for Type {
	#[inline]
	fn from(interface: Interface) -> Self {
		Type::Interface(interface)
	}
}

impl From<Union> for Type {
	#[inline]
	fn from(union: Union) -> Self {
		Type::Union(union)
	}
}

impl From<Subscription> for Type {
	#[inline]
	fn from(subscription: Subscription) -> Self {
		Type::Subscription(subscription)
	}
}
