use litplot::*;
use litplot::plotly::*;

#[test]
fn formatting() {
	let js = js_object! {
		"int" => js_literal!(1),
		"float" => js_literal!(3.14),
		"object" => js_object_val! {
			"string" => js_str!("Hello World"),
			"array" => js_array_val![js_str!("Hello World"), js_literal!(3.14)]
		}
	};

	println!("{}", js)
}