use std::path::PathBuf;

pub struct Options {
	pub output: Option<PathBuf>,
	pub has_title: bool,
	pub has_delimiters: bool,
	pub has_small_index: bool,
	pub has_large_index: bool,
	pub has_author: bool,
	pub has_picture: bool,
}