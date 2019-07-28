use crate::plotly::*;
use crate::error::Error;
use serde::export::PhantomData;
use std::path::Path;
use std::fmt::Debug;

pub trait DataProvider: Debug {
	fn get_data_js(&self) -> String;

	fn save_resources(&self, path: &Path) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct BasicProvider<F, T>
	where F: DataFormat, T: DataType + Transformer<F>
{
	input: String,
	fetch_type: Fetch,
	_phantoms: PhantomData<(F, T)>
}

impl<F, T> BasicProvider<F, T>
	where F: DataFormat, T: DataType + Transformer<F>
{
	pub fn new(fetch_type: Fetch, input: String) -> Self {
		Self { fetch_type, input, _phantoms: PhantomData }
	}
}

impl<F, T> DataProvider for BasicProvider<F, T>
	where F: DataFormat, T: DataType + Transformer<F>
{
	fn get_data_js(&self) -> String {
		format!(
			"{fetch:}(\"{input:}\").then({transform:}).then({prepare:})",
			input = self.input,
			fetch = F::get_fetcher_fn(self.fetch_type),
			transform = T::get_transformer_fn(),
			prepare = T::get_data_fn()
		)
	}

	fn save_resources(&self, path: &Path) -> Result<(), Error> { Ok(()) }
}