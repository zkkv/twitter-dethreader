use twitter_dethreader::options::Options;

use std::{path::PathBuf, process};
use clap::{command, Arg};

fn main() {
	let command = parse_args();

	if let Err(error) = twitter_dethreader::run(&command.tweet_id, &command.options) {
		eprintln!("Error during execution: {}", error);
		process::exit(1);
	}
}

fn parse_args() -> Command {
	let matches = command!()
		.arg(
			Arg::new("tweet-id")
				.required(true)
		)
		.arg(
			Arg::new("output")
				.short('o')
				.long("output")
				.help("Help message")
		)
		.get_matches();

	let tweet_id = matches.get_one::<String>("tweet-id").unwrap().clone();
	
	let output = PathBuf::new()
		.join(matches
		.get_one::<String>("output")
		.unwrap_or(&String::from(".")));

	println!("Output: {:?}", output);

	let options = Options {
		output,
	};

	Command {tweet_id, options}
}

struct Command<> {
	tweet_id: String,
	options: Options,
}