use litplot::plotly::*;

#[test]
fn plot_line() {
	let x = BasicProvider::new("./U4_container.lit".to_string(), Fetcher::Remote(DataFormat::Binary), DataType::Container);

	println!("{}", x.wrap_scope_action("x", "console.log(x);").unwrap());
}