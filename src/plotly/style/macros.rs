macro_rules! plotly_config_struct {
	{
		struct $Name: ident {
			$(
				$(#[$attr: meta])*
				$field: ident: $type: ty
			),*
		}
	} => {
		#[derive(Debug, Clone, Serialize, Builder)]
		pub struct $Name {
			$(
				$(#[$attr])*
				#[builder(setter(into, strip_option), default = "None")]
				#[serde(skip_serializing_if = "Option::is_none")]
				$field: Option<$type>,
			)*
		}

		impl Default for $Name {
			fn default() -> Self {
				Self {
					$($field: None,)*
				}
			}
		}
	}
}