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

	let plot = Plot::new("plot_1".to_string())
		.add_chart(
			LineChartBuilder::default()
				.identifier("chart_1".to_string())
				.data(XYData::new(
					provider_litcontainer(Fetch::Inline, &x, Some("chart_1_x".into())).unwrap(),
					provider_litcontainer(Fetch::Remote, &y, Some("chart_1_y".into())).unwrap(),
				))
				.build()
				.unwrap()
		)
		.add_chart(
			LineChartBuilder::default()
				.identifier("chart_2".to_string())
				.data(XYData::new(
					provider_litcontainer(Fetch::Inline, &x, None).unwrap(),
					provider_litcontainer(Fetch::Remote, &(y + 10.), None).unwrap(),
				))
				.mode(vec![Mode::Markers])
				.build()
				.unwrap()
		)
		.set_style(
			StyleBuilder::default()
				.showlegend(true)
				.legend(
					LegendBuilder::default()
						.orientation(Orientation::Horizontal)
						.build().unwrap()
				)
				.build().unwrap()
		);

	let x = RowVec::regspace_rows(U1, U20, 0.);
	let y = RowVec::regspace_rows(U1, U10, 0.);
	let z = ContainerRM::regspace_rows(U10, U20, 0.) / 20.;

	let plot2 = Plot::new("plot_2".to_string())
		.add_chart(
			HeatmapChartBuilder::default()
				.identifier("chart_3".to_string())
				.data(XYZData::new(
					provider_litcontainer(Fetch::Remote, &x, None).unwrap(),
					provider_litcontainer(Fetch::Remote, &y, None).unwrap(),
					provider_litcontainer(Fetch::Remote, &z, None).unwrap(),
				))
				.build()
				.unwrap()
		)
		.set_style(
			StyleBuilder::default()
				.title("Test Histogram".to_string())
				.legend(LegendBuilder::default().valign(VAlign::Bottom).build().unwrap())
				.build().unwrap()
		);

	let report = Report::new("Test Report")
		.add_node(plot)
		.add_node(plot2);

	println!("{}", report.to_html());

	let in_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join("plot_1");
	if in_path.exists() {
		fs::remove_dir_all(&in_path).unwrap()
	}
	fs::create_dir(&in_path).unwrap();

	report.save(in_path.as_path()).unwrap();
}