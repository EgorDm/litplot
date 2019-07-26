pub mod line;

pub use line::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
	#[serde(alias = "line")]
	Line
}