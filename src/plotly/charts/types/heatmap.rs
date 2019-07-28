use crate::plotly::{ChartBase, XYZData, Chart, ChartData, ChartBuilder, ResourceData, ChartType};
use crate::error::Error;
use std::path::Path;

#[builder(pattern = "owned")]
#[derive(Debug, Serialize, Builder)]
pub struct HeatmapChart<'a> {
	#[builder(default = "ChartType::Heatmap")]
	#[serde(rename = "type")]
	chart_type: ChartType,
	#[serde(flatten)]
	base: ChartBase,
	#[builder(setter(into, strip_option))]
	#[serde(skip_serializing, skip_deserializing)]
	data: Option<XYZData<'a>>,
	#[builder(default = "\"Hot\".to_string()")]
	colorscale: String,
	#[builder(default = "true")]
	showscale: bool,
}

impl<'a> Chart for HeatmapChart<'a> {
	fn identifier(&self) -> &str { self.base.identifier() }

	fn to_js(&self) -> String {
		format!(
			"let {ident:} = {config:}; {ident:}.x = {ident:}_x; {ident:}.y = {ident:}_y; {ident:}.z = {ident:}_z;",
			ident = self.base.identifier(),
			config = serde_json::to_string(self).unwrap()
		)
	}

	fn get_preload_data(&self) -> Vec<(String, String)> {
		self.data.as_ref().unwrap().get_preload_data(self)
	}

	fn chart_type(&self) -> ChartType { self.chart_type }
}

impl<'a> ResourceData for HeatmapChart<'a> {
	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		match self.data {
			Some(ref data) => data.save_resources(path),
			None => Ok(())
		}
	}
}

impl<'a> ChartBuilder for HeatmapChartBuilder<'a>  {
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