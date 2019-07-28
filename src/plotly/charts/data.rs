use crate::plotly::{DataProvider, Chart, ResourceData};
use crate::error::Error;
use std::path::Path;

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

impl<'a> ResourceData for XYData<'a>  {
	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		self.x.save_resources(path)?;
		self.y.save_resources(path)
	}
}



#[derive(Debug)]
pub struct XYZData<'a> {
	x: Box<dyn DataProvider + 'a>,
	y: Box<dyn DataProvider + 'a>,
	z: Box<dyn DataProvider + 'a>,
}

impl<'a> XYZData<'a>  {
	pub fn new<X: DataProvider + 'a, Y: DataProvider + 'a, Z: DataProvider + 'a>(x: X, y: Y, z: Z) -> Self {
		Self {
			x: Box::new(x),
			y: Box::new(y),
			z: Box::new(z),
		}
	}

	pub fn get_x_ident<C: Chart>(&self, c: &C) -> String {
		format!("{}_x", c.identifier())
	}

	pub fn get_y_ident<C: Chart>(&self, c: &C) -> String {
		format!("{}_y", c.identifier())
	}

	pub fn get_z_ident<C: Chart>(&self, c: &C) -> String {
		format!("{}_z", c.identifier())
	}
}

impl<'a> ChartData for XYZData<'a>  {
	fn get_preload_data<C: Chart>(&self, c: &C) -> Vec<(String, String)> {
		vec![
			(self.get_x_ident(c), self.x.get_data_js()),
			(self.get_y_ident(c), self.y.get_data_js()),
			(self.get_z_ident(c), self.z.get_data_js()),
		]
	}
}

impl<'a> ResourceData for XYZData<'a>  {
	fn save_resources(&self, path: &Path) -> Result<(), Error> {
		self.x.save_resources(path)?;
		self.y.save_resources(path)?;
		self.z.save_resources(path)
	}
}