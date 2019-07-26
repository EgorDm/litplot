use crate::plotly::Chart;

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
}

