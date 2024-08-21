mod network;
mod structs;

use std::collections::LinkedList;
use structs::Thread;

pub fn run(tweet_id: &str, destination_dir: &str) -> Result<(), reqwest::Error> {
	let thread = unwrap(tweet_id)?;

	println!("{:?}", thread);

	Ok(())
}

fn unwrap(tweet_id: &str) -> Result<Thread, reqwest::Error> {
	let mut tweet = network::fetch_tweet(tweet_id)?;
	let mut tweets = LinkedList::new();
	
	while let Some(ref parent) = tweet.parent.take() {
		let id = &parent.id;
		tweets.push_front(tweet);
		tweet = network::fetch_tweet(id)?;
	}
	tweets.push_front(tweet);

	Ok(Thread::new(tweets))
}
