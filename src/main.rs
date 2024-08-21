use std::{env, path::Path, process};

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		eprintln!("Include tweet id followed by destination directory");
		process::exit(1);
	}

	let tweet_id = &args[1];
	let destination_dir = Path::new(&args[2]);

	if let Err(error) = twitter_dethreader::run(tweet_id, destination_dir) {
		eprintln!("Error during execution: {}", error);
		process::exit(1);
	}
}