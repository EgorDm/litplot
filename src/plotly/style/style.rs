use serde::{Serialize};
use crate::plotly::{Axis, Calendar, HoverLabel, Margin, Legend, Title};


// TODO: add more options which are in official docs. So much work
plotly_config_struct! {
	struct Style {
		title: String,
		width: i32,
		height: i32,
		margin: Margin,
		hidesources: bool,
		showlegend: bool,
		calendar: Calendar,
		xaxis: Axis,
		yaxis: Axis,
		hoverlabel: HoverLabel,
		legend: Legend
	}
}