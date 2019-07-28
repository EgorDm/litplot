use crate::plotly::*;
use crate::error::Error;
use serde::{Serialize, Deserialize, Serializer};
use std::path::Path;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
	#[serde(rename = "lines")]
	Lines,
	#[serde(rename = "markers")]
	Markers,
	#[serde(rename = "text")]
	Text,
	#[serde(rename = "none")]
	None
}

impl fmt::Display for Mode {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Mode::Lines => write!(f, "lines"),
			Mode::Markers => write!(f, "markers"),
			Mode::Text => write!(f, "text"),
			Mode::None => write!(f, "none"),
		}
	}
}

fn serialize_mode<S>(m: &Vec<Mode>, s: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
{
	let m = m.iter().join("+");
	s.serialize_str(&m)
}


#[builder(pattern = "owned")]
#[derive(Debug, Serialize, Builder)]
pub struct Line<'a> {
	#[builder(default = "ChartType::Line")]
	#[serde(rename = "type")]
	chart_type: ChartType,
	#[serde(flatten)]
	base: ChartBase,
	#[builder(setter(into, strip_option), default = "vec![Mode::Lines]")]
	#[serde(serialize_with = "serialize_mode")]
	mode: Vec<Mode>,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing, skip_deserializing)]
	data: Option<XYData<'a>>
}

impl<'a> Chart for Line<'a> {
	fn identifier(&self) -> &str { self.base.identifier() }

	fn to_js(&self) -> String {
		format!(
			"let {ident:} = {config:}; {ident:}.x = {ident:}_x; {ident:}.y = {ident:}_y;",
			ident = self.base.identifier(),
			config = serde_json::to_string(self).unwrap()
		)
	}

	fn get_preload_data(&self) -> Vec<(String, String)> {
		self.data.as_ref().unwrap().get_preload_data(self)
	}

	fn chart_type(&self) -> ChartType { self.chart_type }
}

impl<'a> ResourceData for Line<'a> {
	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		match self.data {
			Some(ref data) => data.save_resources(path),
			None => Ok(())
		}
	}
}

impl<'a> ChartBuilder for LineBuilder<'a>  {
	fn get_base(&mut self) -> &mut ChartBase {
		match self.base {
			Some(ref mut b) => b,
			None => {
				self.base = Some(ChartBase::default());
				self.base.as_mut().unwrap()
			}
		}
	}
}