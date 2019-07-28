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

	plot.add_chart(
		LineChartBuilder::default()
			.identifier("chart_2".to_string())
			.data(XYData::new(
				provider_litcontainer(Fetch::Inline, &x, None).unwrap(),
				provider_litcontainer(Fetch::Remote, &(y + 10.), None).unwrap(),
			))
			.mode(vec![Mode::Markers])
			.build()
			.unwrap()
	);

	let mut plot2 = Plot::new("plot_2".to_string());

	let x = RowVec::regspace_rows(U1, U20, 0.);
	let y = RowVec::regspace_rows(U1, U10, 0.);
	let z = ContainerRM::regspace_rows(U10, U20, 0.) / 20.;

	plot2.add_chart(
		HeatmapChartBuilder::default()
			.identifier("chart_3".to_string())
			.data(XYZData::new(
				provider_litcontainer(Fetch::Remote, &x, None).unwrap(),
				provider_litcontainer(Fetch::Remote, &y, None).unwrap(),
				provider_litcontainer(Fetch::Remote, &z, None).unwrap(),
			))
			.build()
			.unwrap()
	);

	let report = Report::new("Test Report")
		.add_node(plot)
		.add_node(plot2);

	println!("{}", report.to_html());

	let in_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join("plot_1");
	if !in_path.exists() {
		fs::create_dir(&in_path).unwrap();
	}
	report.save(in_path.as_path()).unwrap();

}