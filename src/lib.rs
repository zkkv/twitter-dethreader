mod network;
mod structs;

use std::{collections::LinkedList, error::Error};
use structs::Thread;

pub fn run(tweet_id: &str, destination_dir: &str) -> Result<(), Box<dyn Error>> {
	let thread = unwrap(tweet_id)?;

	let mut buffer = String::from("# ");

	for tweet in thread.tweets().iter() {
		buffer = format!("{}{}\n\n", buffer, &tweet.text);
	}

	write_to_file(&buffer, &destination_dir)?;

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

fn write_to_file(buffer: &str, destination_dir: &str) -> Result<(), std::io::Error> {
	std::fs::write(format!("{destination_dir}/out.md"), buffer)?;
	Ok(())
}
