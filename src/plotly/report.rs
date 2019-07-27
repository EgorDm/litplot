use crate::plotly::{Plot, ToHtml};
use std::path::Path;
use std::fmt;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

pub enum ReportNode<'a> {
	Plot(Plot<'a>),
	Markdown(String),
	Html(String)
}

impl<'a> From<Plot<'a>> for ReportNode<'a> {
	fn from(p: Plot<'a>) -> Self {
		ReportNode::Plot(p)
	}
}

impl<'a> ToHtml for ReportNode<'a> {
	fn to_html(&self) -> String {
		match self {
			ReportNode::Plot(ref h) => h.to_html(),
			ReportNode::Markdown(ref h) => h.clone(),
			ReportNode::Html(ref h) => h.clone(),
		}
	}
}

pub struct Report<'a> {
	title: String,
	nodes: Vec<ReportNode<'a>>
}

impl<'a> Report<'a> {
	pub fn new(title: &str) -> Self {
		Self { title: title.to_string(), nodes: Vec::new() }
	}

	pub fn add_node<N: Into<ReportNode<'a>>>(self, node: N) -> Self {
		let mut self_mut = self;
		self_mut.nodes.push(node.into());
		self_mut
	}

	pub fn save(&self, path: &Path) -> Result<(), Error> {
		if !path.is_dir() {
			return Err("Path must point to a directory".into())
		}

		let mut f = File::create(path)?;
		write!(f, "{}", self.to_html())?;

		Ok(())
	}
}

impl<'a> ToHtml for Report<'a> {
	fn to_html(&self) -> String {
		let nodes = self.nodes.iter().map(|n| n.to_html()).join("\n");
		format!(
			"<html>\n<head>\n</head>\n<body>\n{nodes:}\n</body>\n</html>",
			nodes = nodes,
		)
	}
}



#[derive(Debug, Clone)]
pub struct Error(String);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.0)
	}
}

impl std::error::Error for Error {}

impl From<&str> for Error {
	fn from(s: &str) -> Self { Self(s.to_string()) }
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self { Self(e.to_string())}
}
