use crate::plotly::{DataProvider, Chart};

pub trait ChartData {
	fn get_preload_data<C: Chart>(&self, c: &C) -> Vec<(String, String)>;
}

#[derive(Debug)]
pub struct XYData<'a> {
	x: Box<dyn DataProvider + 'a>,
	y: Box<dyn DataProvider + 'a>,
}

impl<'a> XYData<'a>  {
	pub fn new<X: DataProvider + 'a, Y: DataProvider + 'a>(x: X, y: Y) -> Self {
		Self {
			x: Box::new(x),
			y: Box::new(y),
		}
	}

	pub fn get_x_ident<C: Chart>(&self, c: &C) -> String {
		format!("{}_x", c.identifier())
	}

	pub fn get_y_ident<C: Chart>(&self, c: &C) -> String {
		format!("{}_y", c.identifier())
	}
}

impl<'a> ChartData for XYData<'a>  {
	fn get_preload_data<C: Chart>(&self, c: &C) -> Vec<(String, String)> {
		vec![
			(self.get_x_ident(c), self.x.get_data_js()),
			(self.get_y_ident(c), self.y.get_data_js())
		]
	}
}