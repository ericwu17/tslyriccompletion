use edit_distance::edit_distance;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Serialize;
use crate::song::{Line, Song};

#[derive(Debug, Clone, Serialize)]
pub struct Question {
	pub shown_line: String,
	pub song: Song,
	pub answer: String,
}

fn lowercase_edit_dist(a: &str, b: &str) -> usize {
	edit_distance(&a.to_lowercase(), &b.to_lowercase())
}

pub fn optimal_truncated_dist(l1: &str, l2: &str) -> (i32, usize) {
	// first we will see how many characters we can truncate from the end of the guess to minimize lowercase_edit_dist(userGuess, answer)

	let mut optimal_k = 0;
	let mut minimal_dist = lowercase_edit_dist(l1, l2);
	let mut k: i32 = 1;
	while l1.len() as i32 - k >= 0 {
		let d = lowercase_edit_dist(&l1[..(l1.len()-k as usize)], l2);
		if d < minimal_dist {
			optimal_k = k;
			minimal_dist = d;
		}
		k += 1;
		// This part is needed so that k always ends on a character boundary, since slices operate on bytes, not characters.
		while l1.len() as i32 - k >= 0 && !l1.is_char_boundary(l1.len()-k as usize) {
			k += 1;
		}
	}
	return (optimal_k, minimal_dist)

}


pub fn are_close_enough(s1: &str, s2: &str) -> bool {
	(edit_distance(s1, s2) as f32 / std::cmp::min(s1.chars().count(), s2.chars().count()) as f32) < {0.1 as f32}
}

fn is_acceptable_guess(guess: &Line) -> bool {
	!guess.has_bad_successor && !guess.has_multiple_successors && !guess.is_exclamatory
}

pub fn pick_distractors (correct_answer: &str, songs: &Vec<Song>) -> Vec<String> {
	const NUM_DISTRACTORS: i32 = 16;
	let mut distractors: Vec<String> = vec![];
	for _ in 0..NUM_DISTRACTORS {
		let random_song = songs.choose(&mut rand::thread_rng()).unwrap();
		let mut random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap().text.as_str();
		while are_close_enough(random_line, correct_answer) {
			random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap().text.as_str();
		}
		distractors.push(random_line.to_owned());
	}

	// This is an easter egg, where there's a small probability for one of the distractors to be a funny quote by Ms. Swift:
	if rand::thread_rng().gen::<i32>() % 100 == 0 {
		distractors[0] = "umm I think for me...".to_owned();
	}

	distractors
}



pub fn pick_random_guess(songs: &Vec<Song>, songs_to_include: &Vec<(String, String)>) -> Question {
	let songs: Vec<Song> = songs.clone().into_iter().filter(|song| songs_to_include.contains(&(song.album.clone(), song.name.clone()))).collect();
	let random_song = songs.choose(&mut rand::thread_rng()).unwrap();

	let candidate_lines = &random_song.lines[0..(random_song.lines.len()-1)];

	let mut random_line = candidate_lines.choose(&mut rand::thread_rng()).unwrap();
	while !is_acceptable_guess(random_line) {
		random_line = candidate_lines.choose(&mut rand::thread_rng()).unwrap();
	}
	let line_num = (&random_song.lines).into_iter().position(|r| r == random_line).unwrap();
	let answer = &random_song.lines[line_num + 1];

	Question {
		shown_line: random_line.text.clone(),
		answer: answer.text.clone(),
		song: random_song.clone(),
	}

}