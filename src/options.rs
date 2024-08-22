use std::path::PathBuf;

pub struct Options {
	pub output: Option<PathBuf>,
	pub has_title: bool,
	pub has_delimiters: bool,
}