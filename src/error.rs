use std::fmt;

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

impl From<litio::Error> for Error {
	fn from(e: litio::Error) -> Self { Self(e.to_string())}
}
