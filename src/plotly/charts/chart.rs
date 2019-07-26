use std::fmt::Debug;
use crate::plotly::ChartBuilder;

pub trait Chart: Debug {
	fn identifier(&self) -> &str;

	fn to_js(&self) -> String;

	fn get_preload_data(&self) -> Vec<(String, String)>;
}