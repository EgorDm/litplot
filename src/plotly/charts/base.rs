use serde::{Serialize, Serializer, Deserialize};
use super::types::ChartType;

#[derive(Debug, Clone, Deserialize)]
pub enum Visibility {
	#[serde(alias = "true")]
	Visible,
	#[serde(alias = "false")]
	Invisible,
	#[serde(alias = "legendonly")]
	LegendOnly
}

impl Default for Visibility {
	fn default() -> Self { Visibility::Visible }
}

impl Serialize for Visibility {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		match self {
			Visibility::Visible => s.serialize_bool(true),
			Visibility::Invisible => s.serialize_bool(false),
			Visibility::LegendOnly => s.serialize_str("legendonly"),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct BaseChart {
	#[serde(skip_serializing)]
	identifier: String,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<String>,
	#[serde(rename = "type")]
	chart_type: ChartType,
	#[builder(default = "1.")]
	opacity: f32,
	visibility: Visibility,
	#[builder(default = "true")]
	showlegend: bool,
	#[builder(setter(into, strip_option))]
	legendgroup: Option<String>
}