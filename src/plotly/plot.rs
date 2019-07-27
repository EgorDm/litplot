use crate::plotly::{Chart, ToHtml};
use itertools::Itertools;

pub struct Plot<'a> {
	identifier: String,
	charts: Vec<Box<dyn Chart + 'a>>
}

impl<'a> Plot<'a> {
	pub fn new(identifier: String) -> Self {
		Self {
			identifier,
			charts: Vec::new()
		}
	}

	pub fn add_chart<C: Chart + 'a>(&mut self, chart: C) {
		self.charts.push(Box::new(chart))
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

		format!(
			"async function {ident:}() {{ {preload:} {configs:} Plotly.newPlot('{ident:}', [{charts}], {{}}); }} {ident:}();",
			ident = self.identifier,
			preload = preload,
			configs = configs,
			charts = charts,
		)
	}

	pub fn get_partial_html(&self) -> String {
		format!(
			"<div id=\"{ident:}\" class=\"litplot-plotly\"></div>",
			ident = self.identifier
		)
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
