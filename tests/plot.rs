use litplot::plotly::*;

#[test]
fn plot_line() {
	let x = BasicProvider::<Binary, Litcontainer>::new(Fetch::Remote, "./U4_container.lit".to_string());

	println!("{}", x.get_data_js());
}

#[test]
fn plot() {
	let mut plot = Plot::new("plot_1".to_string());
	plot.add_chart(
		LineChartBuilder::default()
			.identifier("chart_1".to_string())
			.data(XYData::new(
				BasicProvider::<Binary, Litcontainer>::new(Fetch::Remote, "./U4_container.lit".to_string()),
				BasicProvider::<Binary, Litcontainer>::new(Fetch::Remote, "./U4_container.lit".to_string())
			))
			.build()
			.unwrap()
	);

	println!("{}", plot.get_js());

}