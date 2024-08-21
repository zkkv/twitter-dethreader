mod network;
mod structs;

use std::collections::LinkedList;
use structs::Thread;

pub fn run(tweet_id: &str, destination_dir: &str) -> Result<(), reqwest::Error> {
	let thread = unwrap(tweet_id)?;

	let mut buffer = String::new();

	for tweet in thread.tweets().iter() {
		buffer += &tweet.text;
	}

	println!("{}", buffer);

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
