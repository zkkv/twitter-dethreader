use std::{collections::HashMap, env};
use reqwest::{blocking::{get, Client, RequestBuilder}, header::{HeaderMap, HeaderValue}};

const URL: &str = "https://cdn.syndication.twimg.com/tweet-result";
const USER_AGENT: &str = "curl/8.6.0";

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

	let body = res.text().unwrap();
	println!("{:?}", body);
}
