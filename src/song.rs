
#[derive(Debug, Clone)]
pub struct Song {
	pub album: String,
	pub name: String,
	pub lyrics_raw: String,
	pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {
	pub text: String,
	pub is_exclamatory: bool,
}

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
		}
	}
}

impl Song {
	pub fn new(album: String, name: String, lyrics_raw: String) -> Self {
		let v: Vec<Line> = lyrics_raw.split("\n").filter(|x| !(x.starts_with("[") || x == &""))
			.map(|x| x.trim())
			.map(|x| Line::new(x))
			.collect();
		
		Song {
			album, name, lyrics_raw, lines: v,
		}
	}
}