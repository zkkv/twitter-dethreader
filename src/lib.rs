mod network;
mod structs;
pub mod options;

use structs::{QuotedTweet, Thread, Tweet};
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
	let size = thread.tweets().len();

	if options.has_picture {
		// This should never panic because there's at least one tweet
		let picture = &thread.tweets().front().unwrap().author.profile_img_url;
		buffer += &format!("[Profile picture]({picture})\n\n");
	}

	if options.has_author {
		let name = &thread.tweets().front().unwrap().author.name;
		let handle = &thread.tweets().front().unwrap().author.handle;
		buffer += &format!("Author: {name} (@{handle})\n\n");
	}
	
	if options.has_title {
		buffer += "# ";
	}

	for (idx, tweet) in thread.tweets().iter().enumerate() {
		format_tweet(tweet, idx, size, &mut buffer, options);
	}

	buffer
}

fn format_tweet(tweet: &Tweet, idx: usize, size: usize, mut buffer: &mut String, options: &Options) {
	let idx = idx + 1;
	if options.has_large_index && !(idx == 1 && options.has_title) {
		*buffer += &format!("## [{idx}/{size}]\n\n");
	}

	*buffer += &tweet.text;

	if options.has_small_index {
		*buffer += &format!(" --[{idx}/{size}]");
	}

	*buffer += "\n\n";

	format_tweet_media(tweet, &mut buffer);

	if let Some(quoted_tweet) = &tweet.quoted_tweet {
		format_quoted_tweet(quoted_tweet, &mut buffer);
	}
	
	if options.has_delimiters && idx != size {
		*buffer += "---\n\n";
	}
}

fn format_tweet_media(tweet: &Tweet, buffer: &mut String) {
	for url_entity in tweet.entities.urls.iter() {
			*buffer += &format!("{}\n\n", &url_entity.url);
		}

	if let Some(photos) = &tweet.photos {
			for (photo_idx, photo) in photos.iter().enumerate() {
				*buffer += &format!("[Photo {}]({})\n\n", photo_idx + 1, photo.url);
			}
		}

	if let Some(video) = &tweet.video {
			*buffer += &format!("[Video {}]({})\n\n", 1, video.variants.last().unwrap().url);
		}
}

// Ideally, this should be refactored because it's almost the same code as in format_tweet
fn format_quoted_tweet(tweet: &QuotedTweet, buffer: &mut String) {
	*buffer += &format!("> {}\n\n", tweet.text);

	for url_entity in tweet.entities.urls.iter() {
		*buffer += &format!("> {}\n\n", &url_entity.url);
	}

	if let Some(photos) = &tweet.photos {
		for (photo_idx, photo) in photos.iter().enumerate() {
			*buffer += &format!("> [Quoted photo {}]({})\n\n", photo_idx + 1, photo.url);
		}
	}

	if let Some(video) = &tweet.video {
		*buffer += &format!("[Video {}]({})\n\n", 1, video.variants.last().unwrap().url);
	}
}

fn write_to_file(string: &str, destination: &Option<PathBuf>) -> Result<(), std::io::Error> {
	match destination {
		Some(dest) => std::fs::write(dest, string)?,
		None => std::io::stdout().write_all(string.as_bytes())?
	}
	Ok(())
}
