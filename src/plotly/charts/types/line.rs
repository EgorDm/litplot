use crate::plotly::*;
use crate::error::Error;
use serde::{Serialize, Deserialize};
use std::path::Path;


#[builder(pattern = "owned")]
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct LineChart<'a> {
	#[serde(flatten)]
	base: ChartBase,
	#[builder(setter(into, strip_option), default = "None")]
	#[serde(skip_serializing_if = "Option::is_none")]
	mode: Option<String>,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing, skip_deserializing)]
	data: Option<XYData<'a>>
}

impl<'a> Chart for LineChart<'a> {
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

	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		match self.data {
			Some(ref data) => data.save_resources(path),
			None => Ok(())
		}
	}
}

impl<'a>  ChartBuilder for LineChartBuilder<'a>  {
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