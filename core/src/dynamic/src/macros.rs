macro_rules! impl_set_description {
	() => {
		/// Set the description
		#[inline]
		pub fn description(self, description: impl Into<String>) -> Self {
			Self {
				description: Some(description.into()),
				..self
			}
		}
	};
}

macro_rules! impl_set_deprecation {
	() => {
		/// Set the description
		#[inline]
		pub fn deprecation(self, reason: Option<&str>) -> Self {
			Self {
				deprecation: juniper::meta::DeprecationStatus::Deprecated(reason.map(Into::into)),
				..self
			}
		}
	};
}

macro_rules! impl_set_inaccessible {
	() => {
		/// Indicate that an enum is not accessible from a supergraph when using
		/// Apollo Federation
		///
		/// Reference: <https://www.apollographql.com/docs/federation/federated-types/federated-directives/#inaccessible>
		#[inline]
		pub fn inaccessible(self) -> Self {
			Self {
				inaccessible: true,
				..self
			}
		}
	};
}

macro_rules! impl_directive {
	() => {
		/// Attach directive to the entity
		#[inline]
		pub fn directive(mut self, directive: Directive) -> Self {
			self.directives.push(directive);

			self
		}
	};
}
