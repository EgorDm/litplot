use super::*;

impl ToString for ChartType {
	fn to_string(&self) -> String {
		match self {
			ChartType::Line => "line".to_string()
		}
	}
}

pub struct Line {
	identifier: String,
	x: Box<dyn DataProvider>,
	y: Box<dyn DataProvider>,
}

impl Line {
	pub fn child_ident(&self, ident: &str) -> String {
		format!("{}_{}", self.identifier, ident)
	}

	pub fn get_config(&self) -> js::Object {
		js_object!{
			"type" => js_str!(self.get_type()),
			"x" => js_ident!(self.child_ident("x")),
			"y" => js_ident!(self.child_ident("y"))
		}
	}

	pub fn data_provider_list(&self) -> Vec<(String, &dyn DataProvider)> {
		vec![(self.child_ident("x"), self.x.as_ref()), (self.child_ident("y"), self.y.as_ref())]
	}
}

impl Chart for Line {
	fn get_type(&self) -> ChartType { ChartType::Line }

	fn partial_js(&self) -> String {
		let mut ret = format!("Plotly.newPlot('{}', {});", self.identifier, self.get_config());
		for (ident, provider) in self.data_provider_list() {
			ret = provider.wrap_scope_action(&ident, &ret).unwrap();
		}
		ret
	}

	fn partial_html(&self) -> String {
		unimplemented!()
	}

	fn to_html(&self) -> String {
		unimplemented!()
	}
}