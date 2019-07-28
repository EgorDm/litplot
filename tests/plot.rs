use litplot::plotly::*;
use std::path::Path;
use std::fs;
use litcontainers::*;

#[test]
fn plot_line() {
	let x = BasicProvider::<Binary, Litcontainer>::new(Fetch::Remote, "./U4_container.lit".to_string());

	println!("{}", x.get_data_js());
}

#[test]
fn plot() {
	let x = RowVec::regspace_rows(U1, U20, 0.);
	let y = (&x).pow(2);

	let mut plot = Plot::new("plot_1".to_string());
	plot.add_chart(
		LineChartBuilder::default()
			.identifier("chart_1".to_string())
			.data(XYData::new(
				provider_litcontainer(Fetch::Inline, &x, Some("chart_1_x".into())).unwrap(),
				provider_litcontainer(Fetch::Remote, &y, Some("chart_1_y".into())).unwrap(),
			))
			.build()
			.unwrap()
	);

	let report = Report::new("Test Report")
		.add_node(plot);

	println!("{}", report.to_html());

	let in_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join("plot_1");
	if !in_path.exists() {
		fs::create_dir(&in_path).unwrap();
	}
	report.save(in_path.as_path()).unwrap();

}