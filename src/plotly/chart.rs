pub enum ChartType {
	Line
}

pub trait Chart {
	fn get_type(&self) -> ChartType;

	fn partial_js(&self) -> String;

	fn partial_html(&self) -> String;

	fn to_html(&self) -> String;
}