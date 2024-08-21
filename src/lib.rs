mod network;
mod structs;

pub fn run(tweet_id: &str, destination_dir: &str) {
	let tweet = network::fetch_tweet(tweet_id);

	println!("{:?}", tweet);
}
