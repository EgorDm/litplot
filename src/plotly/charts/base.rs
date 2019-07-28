use super::types::ChartType;
use crate::utils;
use serde::{Serialize, Serializer, Deserialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Getters)]
pub struct ChartBase {
	#[serde(skip_serializing)]
	identifier: String,
	#[builder(setter(into, strip_option), default = "None")]
	#[serde(skip_serializing_if = "Option::is_none")]
	name: Option<String>,
	#[serde(rename = "type")]
	chart_type: ChartType,
	#[builder(default = "1.")]
	opacity: f32,
	#[builder(default = "Visibility::default()")]
	visibility: Visibility,
	#[builder(default = "true")]
	showlegend: bool,
	#[builder(setter(into, strip_option), default = "None")]
	legendgroup: Option<String>
}

impl Default for ChartBase {
	fn default() -> Self {
		let identifier = utils::random_string(32);
		Self {
			identifier,
			name: None,
			chart_type: ChartType::Line,
			opacity: 1.,
			visibility: Visibility::Visible,
			showlegend: true,
			legendgroup: None
		}
	}
}

pub trait ChartBuilder: Sized {
	fn get_base(&mut self) -> &mut ChartBase;

	#[allow(unused_mut)]
	fn identifier(self, value: String) -> Self {
		let mut new = self;
		new.get_base().identifier = value;
		new
	}

	#[allow(unused_mut)]
	fn name<VALUE: ::std::convert::Into<String>>(self, value: VALUE) -> Self {
		let mut new = self;
		new.get_base().name = Some(value.into());
		new
	}

	#[allow(unused_mut)]
	fn chart_type(self, value: ChartType) -> Self {
		let mut new = self;
		new.get_base().chart_type = value;
		new
	}

	#[allow(unused_mut)]
	fn opacity(self, value: f32) -> Self {
		let mut new = self;
		new.get_base().opacity = value;
		new
	}

	#[allow(unused_mut)]
	fn visibility(self, value: Visibility) -> Self {
		let mut new = self;
		new.get_base().visibility = value;
		new
	}

	#[allow(unused_mut)]
	fn showlegend(self, value: bool) -> Self {
		let mut new = self;
		new.get_base().showlegend = value;
		new
	}

	#[allow(unused_mut)]
	fn legendgroup<VALUE: ::std::convert::Into<String>>(
		self,
		value: VALUE,
	) -> Self {
		let mut new = self;
		new.get_base().legendgroup = Some(value.into());
		new
	}
}