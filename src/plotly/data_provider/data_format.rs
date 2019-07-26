use std::fmt::Debug;

pub trait DataFormat: Debug + Clone {
	fn get_fetcher_fn(fetch_type: Fetch) -> String;
}

pub trait DataType: Debug + Clone {
	fn get_data_fn() -> String;
}

pub trait Transformer<T: DataFormat> {
	fn get_transformer_fn() -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum Fetch {
	Inline,
	Remote
}

#[derive(Debug, Clone, Copy)]
pub struct Binary;

impl DataFormat for Binary {
	fn get_fetcher_fn(fetch_type: Fetch) -> String {
		match fetch_type {
			Fetch::Inline => "fetch_inline_binary_base64".to_string(),
			Fetch::Remote => "fetch_remote_binary".to_string(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Litcontainer;

impl DataType for Litcontainer {
	fn get_data_fn() -> String { "(function (i) {return Array.from(i.data)})".to_string() }
}

impl Transformer<Binary> for Litcontainer {
	fn get_transformer_fn() -> String { "transform_binary_litcontainer".to_string() }
}

