#![allow(unused)]

use std::collections::LinkedList;
use serde::Deserialize;


#[derive(Debug)]
pub struct Thread {
	tweets: LinkedList<Tweet>
}

impl Thread {
	pub fn new(tweets: LinkedList<Tweet>) -> Self {
		Self {tweets}
	}

	pub fn tweets(&self) -> &LinkedList<Tweet> {
		&self.tweets
	}
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
	#[serde(rename = "id_str")]
	pub id: String,
	pub text: String,
	pub parent: Option<ParentTweet>,
	#[serde(rename = "user")]
	pub author: Author,
	pub photos: Option<Vec<Photo>>,
}

#[derive(Debug, Deserialize)]
pub struct ParentTweet {
	#[serde(rename = "id_str")]
	pub id: String
}

#[derive(Debug, Deserialize)]
pub struct Author {
	#[serde(rename = "id_str")]
	pub id: String,
	pub name: String,
	#[serde(rename = "screen_name")]
	pub handle: String,
	#[serde(rename = "profile_image_url_https", default)]
	pub profile_img_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Photo {
	pub url: String,
}