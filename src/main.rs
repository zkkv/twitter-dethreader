use twitter_dethreader::options::Options;

use std::{path::PathBuf, process};
use clap::{command, Arg, ArgAction};

fn main() {
	let command = parse_args();

	if let Err(error) = twitter_dethreader::run(&command.tweet_id, &command.options) {
		eprintln!("Error during execution: {}", error);
		process::exit(1);
	}
}

fn parse_args() -> Command {
	let matches = command!()
		.name("Twitter Dethreader")
		.about("Converts Twitter threads into markdown format")
		.author("zkkv")
		.disable_version_flag(true)
		.arg(
			Arg::new("tweet-id")
				.required(true)
				.help("Id of the last tweet in the thread. It can be found in the URL after status.")
		)
		.arg(
			Arg::new("file")
				.short('o')
				.long("output")
				.help("Specify output file (prints to stdout if not specified)")
		)
		.arg(
			Arg::new("title")
				.short('T')
				.long("title")
				.action(ArgAction::SetTrue)
				.help("Make the first tweet into an H1 heading")
		)
		.arg(
			Arg::new("delimiter")
				.short('d')
				.long("delimiter")
				.action(ArgAction::SetTrue)
				.help("Add horizontal lines between each tweet")
		)
		.arg(
			Arg::new("small-index")
				.short('i')
				.long("small-index")
				.action(ArgAction::SetTrue)
				.help("End each tweet with plaintext containing its index (position)")
		)
		.arg(
			Arg::new("large-index")
				.short('I')
				.long("large-index")
				.action(ArgAction::SetTrue)
				.help("Start each tweet with a subheading containing its index (position)")
		)
		.get_matches();

	let tweet_id = matches.get_one::<String>("tweet-id").unwrap().clone();
	
	let output = match matches.get_one::<String>("file") {
		Some(string) => Some(PathBuf::from(string)),
		None => None
	};
	let has_title = matches.get_flag("title");
	let has_delimiters = matches.get_flag("delimiter");
	let has_small_index = matches.get_flag("small-index");
	let has_large_index = matches.get_flag("large-index");

	let options = Options {
		output,
		has_title,
		has_delimiters,
		has_small_index,
		has_large_index,
	};

	Command {tweet_id, options}
}

struct Command<> {
	tweet_id: String,
	options: Options,
}