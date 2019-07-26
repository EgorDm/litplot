use serde::{Serialize, Deserialize};
use crate::plotly::charts::{Chart, BaseChart, ChartData};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct LineChart {
	#[serde(flatten)]
	base: BaseChart,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing_if = "Option::is_none")]
	mode: Option<String>,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing, skip_deserializing)]
	data: Option<ChartData>
}

impl Chart for LineChart {}