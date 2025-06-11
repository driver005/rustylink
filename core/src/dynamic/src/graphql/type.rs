use super::{
	Argument, Arguments, Enum, ExecutionResult, Executor, GraphQLType, GraphQLValueAsync,
	Interface, JuniperField, JuniperTypeRef, MetaType, Object, Registry, Scalar, SelectionSet,
	Subscription, Union,
};
use crate::{BoxFieldFutureJson, ContextBase, FieldValue, ObjectAccessor, SeaResult, Value};
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use std::{
	collections::BTreeMap,
	sync::{Arc, RwLock},
};

pub static TYPE_REGISTRY: Lazy<TypeRegistry> = Lazy::new(TypeRegistry::new);

#[derive(Debug)]
pub struct TypeRegistry {
	types: RwLock<BTreeMap<String, Arc<Type>>>,
}

impl TypeRegistry {
	pub fn new() -> Self {
		Self {
			types: RwLock::new(BTreeMap::new()),
		}
	}
	pub fn clear(&self) {
		let mut types = match self.types.write() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		types.clear();
	}

	pub fn add(&self, name: String, ty: Type) {
		let mut types = match self.types.write() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		if types.insert(name.clone(), Arc::new(ty)).is_some() {
			panic!("Type `{}` could not be added.", name)
		}
	}

	pub fn get(&self, name: &str) -> Option<Arc<Type>> {
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};

		types.get(name).map(|ty| ty.clone())
	}

	pub fn get_filtered<F>(&self, mut f: F) -> BTreeMap<String, Arc<Type>>
	where
		F: FnMut(&str, &Arc<Type>) -> bool,
	{
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		types
			.iter()
			.filter(|(name, ty)| f(name, ty))
			.map(|(name, ty)| (name.clone(), ty.clone()))
			.collect()
	}

	pub fn print(&self) {
		let types = match self.types.read() {
			Ok(types) => types,
			Err(e) => panic!("Failed access types: {}", e),
		};
		println!("types: {:?}", types)
	}
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
	pub(crate) fn as_union(&self) -> Option<&Union> {
		if let Type::Union(union) = self {
			Some(union)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn as_scalar(&self) -> Option<&Union> {
		if let Type::Union(union) = self {
			Some(union)
		} else {
			None
		}
	}

	#[inline]
	pub(crate) fn as_enum(&self) -> Option<&Enum> {
		if let Type::Enum(en) = self {
			Some(en)
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
		selection_set: &'a SelectionSet,
		arguments: &'a ObjectAccessor<'a>,
		parent_value: Option<&'a FieldValue<'a>>,
	) -> Vec<BoxFieldFutureJson<'a>> {
		match self {
			Type::Scalar(scalar) => Vec::from([scalar.collect()]),
			Type::Object(object) => object.collect(ctx, selection_set, arguments, parent_value),
			Type::Enum(en) => Vec::from([en.collect()]),
			Type::Interface(interface) => Vec::from([interface.collect()]),
			Type::Union(union) => Vec::from([union.collect()]),
			Type::Subscription(subscription) => todo!(),
		}
	}

	pub fn to_value(&self, value: &Value) -> SeaResult<Value> {
		match self {
			Type::Enum(en) => en.to_value(value),
			Type::Scalar(scalar) => scalar.to_value(value),
			name => panic!("Type `{}` is not of type Scalar or Enum", name.name()),
		}
	}

	pub fn check(&self, type_name: &str) -> SeaResult<()> {
		match self {
			Type::Interface(interface) => interface.check(type_name),
			Type::Union(union) => union.check(type_name),
			Type::Object(object) => object.check(type_name),
			_ => Ok(()),
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

// Register
impl Type {
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
			Type::Scalar(scalar) => registry.field::<Vec<Scalar>>(field_name, scalar),
			Type::Object(object) => registry.field::<Vec<Object>>(field_name, object),
			Type::Enum(e) => registry.field::<Vec<Enum>>(field_name, e),
			Type::Interface(interface) => registry.field::<Vec<Interface>>(field_name, interface),
			Type::Union(union) => registry.field::<Vec<Union>>(field_name, union),
			Type::Subscription(subscription) => unimplemented!(),
		}
	}

	pub(crate) fn field_null_vec<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.field::<Vec<Option<Scalar>>>(field_name, scalar),
			Type::Object(object) => registry.field::<Vec<Option<Object>>>(field_name, object),
			Type::Enum(e) => registry.field::<Vec<Option<Enum>>>(field_name, e),
			Type::Interface(interface) => {
				registry.field::<Vec<Option<Interface>>>(field_name, interface)
			}
			Type::Union(union) => registry.field::<Vec<Option<Union>>>(field_name, union),
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

	pub(crate) fn field_null_vec_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> JuniperField<'a, Value> {
		match self {
			Type::Scalar(scalar) => {
				registry.field::<Option<Vec<Option<Scalar>>>>(field_name, scalar)
			}
			Type::Object(object) => {
				registry.field::<Option<Vec<Option<Object>>>>(field_name, object)
			}
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
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
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
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
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
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
		}
	}

	pub(crate) fn argument_null_vec<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Vec<Option<Scalar>>>(field_name, scalar),
			Type::Object(object) => registry.arg::<Vec<Option<Object>>>(field_name, object),
			Type::Enum(e) => registry.arg::<Vec<Option<Enum>>>(field_name, e),
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
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
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
		}
	}

	pub(crate) fn argument_null_vec_null<'a>(
		&self,
		field_name: &str,
		registry: &mut Registry<'a, Value>,
	) -> Argument<'a, Value> {
		match self {
			Type::Scalar(scalar) => registry.arg::<Option<Vec<Option<Scalar>>>>(field_name, scalar),
			Type::Object(object) => registry.arg::<Option<Vec<Option<Object>>>>(field_name, object),
			Type::Enum(e) => registry.arg::<Option<Vec<Option<Enum>>>>(field_name, e),
			Type::Subscription(subscription) => unimplemented!(),
			Type::Interface(_) => panic!("Interface is not a valid argument type"),
			Type::Union(_) => panic!("Union is not a valid argument type"),
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
