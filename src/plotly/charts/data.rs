
#[derive(Debug, Clone)]
pub struct ChartData {
	prefix: Option<String>
}

impl ChartData {
	pub fn get_data_expr(&self, ident: &str) -> String {
		match self.prefix {
			Some(ref pre) => format!("{}_{}", pre, ident),
			None => ident.to_string()
		}
	}
}

