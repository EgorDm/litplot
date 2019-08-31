use litplot::plotly::*;
use std::path::Path;
use litcontainers::*;

#[test]
fn plot_line() {
	let x = BasicProvider::<Binary, Litcontainer>::new(Fetch::Remote, "./U4_container.lit".to_string());

	println!("{}", x.get_data_js());
}

#[test]
fn plot() {
	let x = RowVec::regspace(Size::new(U1, U20), RowAxis, 0.);
	let y = (&x).pow(2);

	let plot = Plot::new("plot_1")
		.add_chart(
			LineBuilder::default()
				.identifier("chart_1")
				.data(XYData::new(
					provider_litcontainer(Fetch::Inline, &x, Some("chart_1_x".into())).unwrap(),
					provider_litcontainer(Fetch::Remote, &y, Some("chart_1_y".into())).unwrap(),
				))
				.build()
				.unwrap()
		)
		.add_chart(
			LineBuilder::default()
				.identifier("chart_2")
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

	let x = RowVec::regspace(Size::new(U1, U20), RowAxis, 0.);
	let y = RowVec::regspace(Size::new(U1, U10), RowAxis, 0.);
	let z = ContainerRM::regspace(Size::new(U10, U20), RowAxis, 0.) / 20.;

	let plot2 = Plot::new("plot_2".to_string())
		.add_chart(
			HeatmapBuilder::default()
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
				.title("Test Histogram")
				.legend(LegendBuilder::default().valign(VAlign::Bottom).build().unwrap())
				.build().unwrap()
		);

	let report = Report::new("Test Report")
		.add_node(plot)
		.add_node(plot2);

	let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join("plot_1");
	report.force_save(path.as_path()).unwrap();
}