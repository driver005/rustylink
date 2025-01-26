pub mod related_attr {
	use bae::FromAttributes;

	/// Operations for RelatedEntity enumeration
	#[derive(Default, FromAttributes)]
	pub struct SeaOrm {
		///
		/// Allows to modify target entity
		///
		/// Required on enumeration variants
		///
		/// If used on enumeration attributes
		/// it allows to specify different
		/// Entity ident
		pub entity: Option<syn::Lit>,
		///
		/// Allows to specify RelationDef
		///
		/// Optional
		///
		/// If not supplied the generated code
		/// will utilize `impl Related` trait
		pub def: Option<syn::Lit>,
	}
}
