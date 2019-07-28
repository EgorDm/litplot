use crate::plotly::{Plot, ToHtml};
use crate::error::*;
use std::path::Path;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::fs;

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

		let mut f = File::create(path.join("index.html"))?;
		write!(f, "{}", self.to_html())?;

		// Copy content TODO: the whole assets dir
		let js = include_str!("../../assets/utils.js");
		let mut f = File::create(path.join("utils.js"))?;
		write!(f, "{}", js)?;

		let assets_path = path.join("assets");
		if !assets_path.exists() {
			fs::create_dir(&assets_path)?;
		}

		for n in &self.nodes {
			match n {
				ReportNode::Plot(p) => p.save_resources(&assets_path)?,
				_ => {}
			}
		}

		Ok(())
	}
}

impl<'a> ToHtml for Report<'a> {
	fn to_html(&self) -> String {
		let head = [
			r#"<script src="./utils.js"></script>"#,
			r#"<script src="https://cdn.plot.ly/plotly-1.2.0.min.js"></script>"#
		].join("\n");
		let nodes = self.nodes.iter().map(|n| n.to_html()).join("\n");
		format!(
			"<html>\n{head:}\n<head>\n</head>\n<body>\n{nodes:}\n</body>\n</html>",
			head = head,
			nodes = nodes,
		)
	}
}



