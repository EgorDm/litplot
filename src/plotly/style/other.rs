#[derive(Debug, Clone, Serialize)]
pub struct Color(String);


plotly_config_struct! {
	struct Font {
		family: String,
		size: i32,
		color: Color
	}
}

#[derive(Debug, Clone, Serialize)]
pub enum Side {
	#[serde(rename = "top")]
	Top,
	#[serde(rename = "bottom")]
	Bottom,
	#[serde(rename = "left")]
	Left,
	#[serde(rename = "right")]
	Right,
}

#[derive(Debug, Clone, Serialize)]
pub enum Align {
	#[serde(rename = "auto")]
	Auto,
	#[serde(rename = "left")]
	Left,
	#[serde(rename = "right")]
	Right,
	#[serde(rename = "center")]
	Center,
}

#[derive(Debug, Clone, Serialize)]
pub enum Orientation {
	#[serde(rename = "v")]
	Vertical,
	#[serde(rename = "h")]
	Horizontal,
}

#[derive(Debug, Clone, Serialize)]
pub enum VAlign {
	#[serde(rename = "top")]
	Top,
	#[serde(rename = "middle")]
	Middle,
	#[serde(rename = "bottom")]
	Bottom,
}

plotly_config_struct! {
	struct HoverLabel {
		bgcolor: Color,
		bordercolor : Color,
		font: Font,
		align: Align,
		namelength: i32
	}
}

plotly_config_struct! {
	struct Margin {
		l: i32,
		r: i32,
		t: i32,
		b: i32,
		pad: i32,
		autoexpand: bool
	}
}

plotly_config_struct! {
	struct Pad {
		t: i32,
		r: i32,
		b: i32,
		l: i32
	}
}

plotly_config_struct! {
	struct Title {
		text: String,
		font: Font
	}
}

impl Title {
	pub fn new(text: String) -> Self {
		TitleBuilder::default().text(text).build().unwrap()
	}
}

