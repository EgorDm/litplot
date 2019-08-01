use crate::plotly::data_provider::*;
use crate::error::Error;
use crate::utils;
use std::marker::PhantomData;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use litcontainers::*;
use serde::Serialize;
use litio::SerializableScalar;

#[derive(Debug, Clone)]
pub struct BinaryProvider<T>
	where T: DataType + Transformer<Binary>
{
	filename: String,
	input: Vec<u8>,
	fetch_type: Fetch,
	_phantoms: PhantomData<(T)>
}

impl<T> BinaryProvider<T>
	where T: DataType + Transformer<Binary>
{
	pub fn new(fetch_type: Fetch, input: Vec<u8>, filename: Option<String>) -> Self {
		let filename = match filename {
			Some(filename) => filename,
			None => utils::random_string(32)
		};
		Self { filename, fetch_type, input, _phantoms: PhantomData }
	}
}

impl<T> DataProvider for BinaryProvider<T>
	where T: DataType + Transformer<Binary>
{
	fn get_data_js(&self) -> String {
		let input = match self.fetch_type {
			Fetch::Inline => base64::encode(&self.input),
			Fetch::Remote => format!("./assets/{}.{}", self.filename, T::extension())
		};

		format!(
			"{fetch:}(\"{input:}\").then({transform:}).then({prepare:})",
			input = input,
			fetch = Binary::get_fetcher_fn(self.fetch_type),
			transform = T::get_transformer_fn(),
			prepare = T::get_data_fn()
		)
	}

	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		match self.fetch_type {
			Fetch::Inline => {},
			Fetch::Remote => {
				let path = path.join(format!("{}.{}", &self.filename, T::extension()));
				File::create(path)?.write(&self.input)?;
			}
		}
		Ok(())
	}
}

pub fn provider_litcontainer<T, R, C, S>(fetch_type: Fetch, storage: &S, filename: Option<String>)
	-> Result<BinaryProvider<Litcontainer>, Error>
	where T: Scalar + SerializableScalar, R: Dim, C: Dim, S: Storage<T, R, C>/* + StorageConstructor<T, R, C>*/
{
	let mut bytes = Vec::new();
	litio::write_binary(&mut bytes, storage)?;
	Ok(BinaryProvider::new(fetch_type, bytes, filename))
}