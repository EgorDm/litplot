use crate::plotly::{Chart, ToHtml, Style};
use crate::error::Error;
use itertools::Itertools;
use std::path::Path;

pub struct Plot<'a> {
	identifier: String,
	charts: Vec<Box<dyn Chart + 'a>>,
	style: Option<Style>
}

impl<'a> Plot<'a> {
	pub fn new<V: Into<String>>(identifier: V) -> Self {
		Self {
			identifier: identifier.into(),
			charts: Vec::new(),
			style: None,
		}
	}

	pub fn add_chart<C: Chart + 'a>(self, chart: C) -> Self {
		let mut self_mut = self;
		self_mut.charts.push(Box::new(chart));
		self_mut
	}

	pub fn set_style(self, style: Style) -> Self {
		let mut self_mut = self;
		self_mut.style = Some(style);
		self_mut
	}

	pub fn get_partial_js(&self) -> String {
		let mut vars = Vec::new();
		let mut exprs = Vec::new();
		for (var, expr) in self.charts.iter().map(|c| c.get_preload_data()).flatten() {
			vars.push(var);
			exprs.push(expr);
		}

		let preload = format!(
			"let [{vars:}] = await Promise.all([{exprs:}]);",
			vars = vars.join(","),
			exprs = exprs.join(",")
		);

		let configs = self.charts.iter().map(|c| c.to_js()).join("\n");
		let charts = self.charts.iter().map(|c| c.identifier()).join(",");

		let style = match self.style {
			Some(ref style) => serde_json::to_string(style).unwrap(),
			None => "{}".to_string()
		};

		format!(
			"async function {ident:}() {{ {preload:} {configs:} Plotly.newPlot('{ident:}', [{charts}], {style:}); }} {ident:}();",
			ident = self.identifier,
			preload = preload,
			configs = configs,
			charts = charts,
			style = style,
		)
	}

	pub fn get_partial_html(&self) -> String {
		format!(
			"<div id=\"{ident:}\" class=\"litplot-plotly\"></div>",
			ident = self.identifier
		)
	}

	pub fn save_resources(&self, path: &Path) -> Result<(), Error> {
		for c in &self.charts {
			c.save_resources(path)?;
		}
		Ok(())
	}
}


impl<'a> ToHtml for Plot<'a> {
	fn to_html(&self) -> String {
		format!(
			"{html:}\n<script>\n{js:}\n</script>",
			html = self.get_partial_html(),
			js = self.get_partial_js(),
		)
	}
}
