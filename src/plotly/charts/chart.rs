use crate::plotly::{ChartType};
use crate::error::Error;
use std::fmt::Debug;
use std::path::Path;

pub trait ResourceData {
	fn save_resources(&self, path: &Path) -> Result<(), Error>;
}

pub trait Chart: Debug + ResourceData {
	fn identifier(&self) -> &str;

	fn to_js(&self) -> String;

	fn get_preload_data(&self) -> Vec<(String, String)>;

	fn chart_type(&self) -> ChartType;
}