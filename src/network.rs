use std::collections::HashMap;
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue}};
use crate::structs::Tweet;

const URL: &str = "https://cdn.syndication.twimg.com/tweet-result";
const USER_AGENT: &str = "curl/8.6.0";

pub fn fetch_tweet(tweet_id: &str) -> Result<Tweet, reqwest::Error> {
	let mut params = HashMap::new();
	params.insert("token", "foo");
	params.insert("lang", "en");
	params.insert("id", tweet_id);

	let mut headers = HeaderMap::new();
	headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

	let response = Client::new()
		.get(URL)
		.query(&params)
		.headers(headers)
		.send()?;

	let tweet: Tweet = response.json()?;
	Ok(tweet)
}