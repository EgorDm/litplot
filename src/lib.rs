#[macro_use]
extern crate typed_html;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_getters;


pub mod plotly;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
