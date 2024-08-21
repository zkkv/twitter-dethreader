use serde::Deserialize;

#[derive(Debug)]
struct Thread {
	tweets: Vec<Tweet>,
	author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
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