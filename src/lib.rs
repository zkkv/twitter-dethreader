mod network;
mod structs;
pub mod options;

use structs::Thread;
use options::Options;

use std::{collections::LinkedList, error::Error, io::Write, path::PathBuf};


pub fn run(tweet_id: &str, options: &Options) -> Result<(), Box<dyn Error>> {
	let thread = unwrap(tweet_id)?;
	let formatted = format_thread(&thread, &options);
	write_to_file(&formatted, &options.output)?;
	Ok(())
}

fn unwrap(tweet_id: &str) -> Result<Thread, reqwest::Error> {
	let mut tweet = network::fetch_tweet(tweet_id)?;
	let mut tweets = LinkedList::new();
	
	while let Some(ref parent) = tweet.parent.take() {
		tweets.push_front(tweet);
		tweet = network::fetch_tweet(&parent.id)?;
	}
	tweets.push_front(tweet);

	Ok(Thread::new(tweets))
}

fn format_thread(thread: &Thread, options: &Options) -> String {
	let mut buffer = String::new();
	
	if options.has_title {
		buffer += "# ";
	}

	for tweet in thread.tweets().iter() {
		buffer = format!("{}{}\n\n", buffer, &tweet.text);
		if options.has_delimiters {
			buffer += "---\n\n";
		}
	}

	buffer
}

fn write_to_file(string: &str, destination: &Option<PathBuf>) -> Result<(), std::io::Error> {
	match destination {
		Some(dest) => std::fs::write(dest, string)?,
		None => std::io::stdout().write_all(string.as_bytes())?
	}
	Ok(())
}
