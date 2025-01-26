use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod derives;

/// The DeriveRelatedEntity derive macro will implement seaography::RelationBuilder for RelatedEntity enumeration.
///
/// ### Usage
///
/// ```ignore
/// use sea_orm::entity::prelude::*;
///
/// // ...
/// // Model, Relation enum, etc.
/// // ...
///
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
/// pub enum RelatedEntity {
///     #[sea_orm(entity = "super::address::Entity")]
///     Address,
///     #[sea_orm(entity = "super::payment::Entity")]
///     Payment,
///     #[sea_orm(entity = "super::rental::Entity")]
///     Rental,
///     #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def()")]
///     SelfRef,
///     #[sea_orm(entity = "super::store::Entity")]
///     Store,
///     #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def().rev()")]
///     SelfRefRev,
/// }
/// ```
#[proc_macro_derive(DeriveRelatedEntity, attributes(sea_orm))]
pub fn derive_related_entity(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	derives::expand_derive_related_entity(input).unwrap_or_else(Error::into_compile_error).into()
}
