pub mod line;
pub mod heatmap;

pub use line::*;
pub use heatmap::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChartType {
	#[serde(rename = "scatter")]
	Line,
	#[serde(rename = "scatter")]
	Scatter,
	#[serde(rename = "heatmap")]
	Heatmap,
}