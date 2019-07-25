use std::collections;
use std::collections::HashMap;
use std::fmt;
use itertools::Itertools;

pub enum Value {
	Identifier(String),
	Literal(String),
	String(String),
	Object(Object),
	Array(Array),
}

impl From<&str> for Value {
	fn from(v: &str) -> Self {
		Value::String(v.to_string())
	}
}

impl From<String> for Value {
	fn from(v: String) -> Self {
		Value::String(v)
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Value::Identifier(s) => write!(f, "{}", s),
			Value::Literal(s) => write!(f, "{}", s),
			Value::String(s) => write!(f, "\"{}\"", s),
			Value::Object(s) => write!(f, "{}", s),
			Value::Array(s) => write!(f, "{}", s),
		}
	}
}

pub struct Object {
	data: HashMap<String, Value>
}

impl Object {
	pub fn new() -> Self { Self { data: HashMap::new() } }

	pub fn add(&mut self, key: String, value: Value) {
		self.data.insert(key, value);
	}

	pub fn get(&self, key: &str) -> Option<&Value> {
		self.data.get(key)
	}

	pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
		self.data.get_mut(key)
	}

	pub fn keys(&self) -> collections::hash_map::Keys<'_, String, Value> {
		self.data.keys()
	}
}

impl Into<Value> for Object {
	fn into(self) -> Value { Value::Object(self) }
}

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let keys = self.data.iter()
			.map(|(k, v)| format!("{}: {}", k, v))
			.join(",");
		write!(f, "{{{}}}", keys)
	}
}

pub struct Array {
	data: Vec<Value>
}

impl Array {
	pub fn new(data: Vec<Value>) -> Self { Self { data } }
}

impl Into<Value> for Array {
	fn into(self) -> Value { Value::Array(self) }
}

impl fmt::Display for Array {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let keys = self.data.iter().join(",");
		write!(f, "[{}]", keys)
	}
}