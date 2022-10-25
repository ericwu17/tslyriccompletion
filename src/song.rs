use std::collections::HashMap;

use serde::Serialize;

use crate::guess_generating::are_close_enough;


#[derive(Debug, Clone, Serialize)]
pub struct Song {
	pub album: String,
	pub name: String,
	pub lyrics_raw: String,
	pub lines: Vec<Line>,
}
unsafe impl Send for Song {}
unsafe impl Sync for Song {}

#[derive(Debug, Clone, Serialize)]
pub struct Line {
	pub text: String,
	pub is_exclamatory: bool,
	pub has_multiple_successors: bool,
	pub has_bad_successor: bool,
}
unsafe impl Send for Line {}
unsafe impl Sync for Line {}

impl PartialEq for Line {
	fn eq(&self, other: &Line) -> bool {
		self.text == other.text
	}
}

impl Line {
	pub fn new(text: &str) -> Line {
		// The goal here is to calculate whether a line is "exclamatory". A line like "Oh, oh, oh, whoa" is exclamatory, since it contains many exclamatory words.
		// We don't want the guessing game's questions to involve exclamatory words, because they are generally difficult to recall or type properly.

		let mut is_exclamatory = false;

		let exclamatory_words = ["mmmm", "mmm", "mm", "oh", "ohh", "la", "na", "no", "my", "uh", "huh", "ahh", "ah", "ha", "yeah", "whoa",
			"ayy", "i", "eh", "hey", "ra", "di", "da"];

		let text_lower = text.to_lowercase();
		let words: Vec<&str> = text_lower.split(|x: char| !x.is_alphabetic()).filter(|x| *x != "").collect();
		let num_words = words.len();
		let exclamatory_words = words.clone().into_iter().filter(|x| exclamatory_words.contains(x)).collect::<Vec<&str>>();
		let num_exclamatory_words = exclamatory_words.len();
		
		if num_exclamatory_words as f32 / num_words as f32 >= 0.5 {
			is_exclamatory = true;
		}
		// we label short lines as exclamatory to exclude them too
		if num_words <= 2 {
			is_exclamatory = true;
		}


		// Nevermind, there was too many false positives with this method. :(
		// // A line is marked as exclamatory if its most common 2 words make up >= 50% of the line's words.
		// let mut word_frequencies: HashMap<&str, i32> = HashMap::new();
		// for word in words {
		// 	if word_frequencies.contains_key(word) {
		// 		word_frequencies.insert(word, word_frequencies.get(word).unwrap() + 1);
		// 	} else {
		// 		word_frequencies.insert(word, 1);
		// 	}
		// }
		// let mut word_freqs: Vec<(&str, i32)> = word_frequencies.into_iter().collect();
		// word_freqs.sort_by(|(_, a2), (_, b2)| a2.partial_cmp(b2).unwrap_or(Ordering::Equal));
		// word_freqs.reverse();
		// let mut num_words_in_two_most_common_words = 0;
		// num_words_in_two_most_common_words += word_freqs.get(0).unwrap_or(&("", 0)).1;
		// num_words_in_two_most_common_words += word_freqs.get(1).unwrap_or(&("", 0)).1;

		// if num_words_in_two_most_common_words as f32 / num_words as f32 >= 0.5 {
		// 	is_exclamatory = true;
		// }


		Line {
			text: text.to_owned(),
			is_exclamatory,
			has_multiple_successors: false,
			has_bad_successor: false,
		}
	}
}

impl Song {
	pub fn new(album: String, name: String, lyrics_raw: String) -> Self {
		let mut lines: Vec<Line> = lyrics_raw.split("\n").filter(|x| !(x.starts_with("[") || x == &""))
			.map(|x| x.trim())
			.map(|x| Line::new(x))
			.collect();

		for index in 0..lines.len()-1 {
			if lines[index+1].is_exclamatory {
				lines[index].has_bad_successor = true;
			}
		}
		let n = lines.len();
		lines[n-1].has_bad_successor = true;


		let mut continuation_map: HashMap<String, Vec<String>> = HashMap::new();
		for index in 0..lines.len()-1 {
			let line = &lines[index];
			if let Some(continuations) = continuation_map.get(&line.text) {
				if !continuations.contains(&lines[index+1].text) {
					let mut v = continuations.clone();
					v.push(lines[index+1].text.clone());
					continuation_map.insert(line.text.clone(), v);
				}
			} else {
				continuation_map.insert(line.text.clone(), vec![lines[index+1].text.clone()]);
			}
		}
		for index in 0..lines.len()-1 {
			if let Some(continuations) = continuation_map.get(&lines[index].text) {
				if continuations.len() > 1 {
					lines[index].has_multiple_successors = true;
				}
			}
		}


		Song {
			album, name, lyrics_raw, lines,
		}
	}
}

fn line_has_multiple_successors(guess: &Line, lines: &Vec<Line>) -> bool {

	let mut possible_continuations = vec![];
	for (index, line) in (&lines[..lines.len()-1]).into_iter().enumerate() {
		if are_close_enough(line.text.as_str(), guess.text.as_str()) {
			possible_continuations.push(lines[index+1].clone());
		}
	}
	for continuation in &possible_continuations {
		if continuation.is_exclamatory {
			return true;
		}
	}

	for c1 in &possible_continuations {
		for c2 in &possible_continuations {
			if c1 != c2 {
				return true;
			}
		}
	}

	false
}
