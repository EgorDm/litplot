#[derive(Debug, Clone, Copy)]
pub enum Fetcher {
	Inline(DataFormat),
	Remote(DataFormat)
}

impl Fetcher {
	pub fn fetch_fn(&self) -> Option<String> {
		match self {
			Fetcher::Inline(DataFormat::Binary) => Some("fetch_inline_binary_base64".to_string()),
			Fetcher::Remote(DataFormat::Binary) => Some("fetch_remote_binary".to_string()),
			_ => None
		}
	}

	pub fn get_format(&self) -> DataFormat {
		match self {
			Fetcher::Inline(ret) => *ret,
			Fetcher::Remote(ret) => *ret,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum DataFormat {
	Binary,
	Text
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
	Container
}

pub struct Transformer(DataFormat, DataType);

impl Transformer {
	pub fn transform_fn(&self) -> Option<String> {
		match self {
			Transformer(DataFormat::Binary, DataType::Container) => Some("transform_binary_litcontainer".to_string()),
			_ => None
		}
	}
}


pub trait DataProvider {
	fn wrap_scope_action(&self, identifier: &str, action: &str) -> Result<String, ()>;
}

pub struct BasicProvider {
	value: String,
	fetcher: Fetcher,
	data_type: DataType
}

impl BasicProvider {
	pub fn new(value: String, fetcher: Fetcher, data_type: DataType) -> Self {
		Self { value, fetcher, data_type }
	}
}

impl DataProvider for BasicProvider {
	fn wrap_scope_action(&self, identifier: &str, action: &str) -> Result<String, ()> {
		let action = format!("function ({}) {{ {} }}", identifier, action);
		let transformer = Transformer(self.fetcher.get_format(), self.data_type).transform_fn().ok_or(())?;
		let fetcher = self.fetcher.fetch_fn().ok_or(())?;

		Ok(format!("{fetcher:}(\"{value:}\", function({ident:}_raw) {{ {transformer:}({ident:}_raw, {action:}) }})",
		        value = &self.value,
		        transformer = transformer,
		        fetcher = fetcher,
		        ident = identifier,
		        action = action,
		))
	}
}