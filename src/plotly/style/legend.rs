use crate::plotly::{Color, Font, Orientation, Align, VAlign};
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Serialize)]
pub enum ItemSizing {
	#[serde(rename = "trace")]
	Trace,
	#[serde(rename = "constant")]
	Constant,
}

#[derive(Debug, Clone)]
pub enum ItemClick {
	Toggle,
	ToggleOthers,
	False
}

impl Serialize for ItemClick {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
		S: Serializer {
		match self {
			ItemClick::Toggle => serializer.serialize_str("toggle"),
			ItemClick::ToggleOthers => serializer.serialize_str("toggleothers"),
			ItemClick::False => serializer.serialize_bool(false)
		}
	}
}

plotly_config_struct! {
	struct Legend {
		bgcolor: Color,
		bordercolor: Color,
		borderwidth: i32,
		font: Font,
		orientation: Orientation,
		traceorder: String,
		tracegroupgap: i32,
		itemsizing: ItemSizing,
		itemclick: ItemClick,
		itemdoubleclick: ItemClick,
		x: f32,
		xanchor: Align,
		y: f32,
		yanchor: Align,
		uirevision: String,
		valign: VAlign
	}
}