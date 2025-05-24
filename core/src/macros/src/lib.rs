use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, parse_macro_input};

mod derives;
// #[macro_use]
// pub mod macros;

// --- exmaple -----------------------
//
//    async fn value_from_db(
//        &self,
//        ctx: &Context<'_>,
//        #[graphql(desc = "Id of object")] id: i64
//    ) -> Result<String> {
//        let conn = ctx.data::<DbPool>()?.take();
//        Ok(conn.query_something(id)?.name)
//    }
//
//
//#[Object]
//impl Query {
//    #[field(desc = "\"me: Single-line comment\"")]
//    pub async fn me(&self, ctx: &Context<'_>) -> Me {
//        ctx.data_unchecked::<DataSource>().me()
//    }
//    pub async fn active(&self) -> bool {
//        self.active.clone()
//    }
//}
#[proc_macro_derive(NameString)]
pub fn impl_name(input: TokenStream) -> TokenStream {
	let DeriveInput {
		ident,
		..
	} = syn::parse_macro_input!(input);
	let struct_name = ident;

	let expand = quote! {
		impl NameString for #struct_name {
			fn name_string(&self) -> String {
				self.name.to_string()
			}
		}

		impl NameString for &#struct_name {
			fn name_string(&self) -> String {
				self.name.to_string()
			}
		}
	};

	TokenStream::from(expand)
}

#[proc_macro_derive(LinePosition)]
pub fn impl_line_pos(input: TokenStream) -> TokenStream {
	let DeriveInput {
		ident,
		..
	} = syn::parse_macro_input!(input);
	let struct_name = ident;

	let expand = quote! {
		impl LinePosition for #struct_name {
			fn line_position(&self) -> usize {
				self.line_pos
			}
		}

		impl LinePosition for &#struct_name {
			fn line_position(&self) -> usize {
				self.line_pos
			}
		}
	};

	TokenStream::from(expand)
}

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
