use serde::{Serialize, Serializer};
use crate::plotly::style::Title;
use crate::plotly::{Color, Side};

#[derive(Debug, Clone, Serialize)]
pub enum AxisType {
	#[serde(rename = "-")]
	Default,
	#[serde(rename = "linear")]
	Linear,
	#[serde(rename = "log")]
	Log,
	#[serde(rename = "date")]
	Date,
	#[serde(rename = "category")]
	Category,
}

#[derive(Debug, Clone, Serialize)]
pub enum RangeMode {
	#[serde(rename = "normal")]
	Normal,
	#[serde(rename = "tozero")]
	ToZero,
	#[serde(rename = "nonnegative")]
	NonNegative,
}

#[derive(Debug, Clone, Serialize)]
pub enum TickMode {
	#[serde(rename = "auto")]
	Auto,
	#[serde(rename = "linear")]
	Linear,
	#[serde(rename = "array")]
	Array,
}

#[derive(Debug, Clone, Serialize)]
pub enum Layer {
	#[serde(rename = "above traces")]
	AboveTraces,
	#[serde(rename = "below traces")]
	BelowTraces,
}

#[derive(Debug, Clone, Serialize)]
pub enum Calendar {
	#[serde(rename = "Gregorian")]
	Gregorian,
	#[serde(rename = "Chinese")]
	Chinese,
	#[serde(rename = "Coptic")]
	Coptic,
	#[serde(rename = "discworld")]
	Discworld,
	#[serde(rename = "ethiopian")]
	Ethiopian,
	#[serde(rename = "hebrew")]
	Hebrew,
	#[serde(rename = "islamic")]
	Islamic,
	#[serde(rename = "julian")]
	Julian,
	#[serde(rename = "mayan")]
	Mayan,
	#[serde(rename = "nanakshahi")]
	Nanakshahi,
	#[serde(rename = "nepali")]
	Nepali,
	#[serde(rename = "persian")]
	Persian,
	#[serde(rename = "jalali")]
	Jalali,
	#[serde(rename = "taiwan")]
	Taiwan,
	#[serde(rename = "thai")]
	Thai,
	#[serde(rename = "ummalqura")]
	Ummalqura,
}

#[derive(Debug, Clone, Serialize)]
pub enum CategoryOrder {
	#[serde(rename = "trace")]
	Trace,
	#[serde(rename = "category ascending")]
	CategoryAscending,
	#[serde(rename = "category descending")]
	CategoryDescending,
	#[serde(rename = "array")]
	Array,
	#[serde(rename = "total ascending")]
	TotalAscending,
	#[serde(rename = "total descending")]
	TotalDescending,
	#[serde(rename = "min ascending")]
	MinAscending,
	#[serde(rename = "min descending")]
	MinDescending,
	#[serde(rename = "max ascending")]
	MaxAscending,
	#[serde(rename = "max descending")]
	MaxDescending,
	#[serde(rename = "sum ascending")]
	SumAscending,
	#[serde(rename = "sum descending")]
	SumDescending,
	#[serde(rename = "mean ascending")]
	MeanAscending,
	#[serde(rename = "mean descending")]
	MeanDescending,
	#[serde(rename = "median ascending")]
	MedianAscending,
	#[serde(rename = "median descending")]
	MedianDescending,
}

#[derive(Debug, Clone)]
pub enum AutoRange {
	True,
	False,
	Reversed,
}

impl Serialize for AutoRange {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
		S: Serializer
	{
		match self {
			AutoRange::True => serializer.serialize_bool(true),
			AutoRange::False => serializer.serialize_bool(false),
			AutoRange::Reversed => serializer.serialize_str("reversed"),
		}
	}
}
//		#[serde(rename = "type")]
plotly_config_struct! {
	struct Axis {
		visible: bool,
		color: Color,
		title: Title,
		#[serde(rename = "type")]
		axis_type: AxisType,
		autorange: AutoRange,
		rangemode: RangeMode,
		range: Vec<serde_json::Value>,
		fixedrange: bool,
		tickmode: TickMode,
		scaleanchor: String,
		scaleratio: f32,
		hoverformat: String,
		showline: bool,
		linecolor: Color,
		linewidth: i32,
		showgrid: bool,
		gridcolor: Color,
		gridwidth: i32,
		zeroline: bool,
		zerolinecolor: Color,
		zerolinewidth: i32,
		showdividers: bool,
		dividercolor: Color,
		dividerwidth: i32,
		anchor: String,
		side: Side,
		overlaying: String,
		layer: Layer,
		domain: f32,
		position: f32,
		categoryorder: CategoryOrder,
		categoryarray: Vec<serde_json::Value>,
		uirevision: String,
		calendar: Calendar
	}
}