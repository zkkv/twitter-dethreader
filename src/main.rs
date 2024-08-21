use std::{collections::HashMap, env};
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};
use serde::Deserialize;

const URL: &str = "https://cdn.syndication.twimg.com/tweet-result";
const USER_AGENT: &str = "curl/8.6.0";

#[derive(Debug)]
struct Thread {
	tweets: Vec<Tweet>,
	author: Author,
}

#[derive(Debug, Deserialize)]
struct Tweet {
	#[serde(rename = "id_str")]
	id: String,
	text: String,
	parent: Option<ParentTweet>,
}

#[derive(Debug, Deserialize)]
struct ParentTweet {
	#[serde(rename = "id_str")]
	id: String
}

#[derive(Debug, Deserialize)]
struct Author {
	id: String,
	name: String,
	handle: String,
	profile_img_url: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let tweet_id = &args[1];
	let destination = &args[2];

	let mut params = HashMap::new();
	params.insert("token", "f");
	params.insert("lang", "en");
	params.insert("id", tweet_id);

	let mut headers = HeaderMap::new();
	headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

	let res = Client::new()
		.get(URL)
		.query(&params)
		.headers(headers)
		.send()
		.expect("Couldn't get a response");

	let body: Tweet = res.json().unwrap();
	println!("{:?}", body);
}
