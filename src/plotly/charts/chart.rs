use crate::plotly::ChartBuilder;
use crate::error::Error;
use std::fmt::Debug;
use std::path::Path;

pub trait Chart: Debug {
	fn identifier(&self) -> &str;

	fn to_js(&self) -> String;

	fn get_preload_data(&self) -> Vec<(String, String)>;

	fn save_resources(&self, path: &Path) -> Result<(), Error>;
}