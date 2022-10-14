use edit_distance::edit_distance;
use rand::seq::SliceRandom;
use crate::Song;

#[derive(Debug)]
pub struct Question {
	pub song_title: String,
	pub song_album: String,
	pub shown_line: String,
	pub answer: String,
}


fn are_close_enough(s1: &str, s2: &str) -> bool {
	(edit_distance(s1, s2) as f32 / std::cmp::min(s1.len(), s2.len()) as f32) < {0.1 as f32}
}

fn is_acceptable_guess(guess: &str, lines: &Vec<String>) -> bool {
	if !lines.contains(&guess.to_owned()) {
		return false;
	}

	let mut possible_continuations = vec![];
	for (index, line) in (&lines[..lines.len()-1]).into_iter().enumerate() {
		if are_close_enough(line, guess) {
			possible_continuations.push(lines[index+1].clone());
		}
	}
	for c1 in &possible_continuations {
		for c2 in &possible_continuations {
			if c1 != c2 {
				return false;
			}
		}
	}

	true
}

pub fn pick_distractors (correct_answer: &str, songs: &Vec<Song>) -> Vec<String> {
	const NUM_DISTRACTORS: i32 = 16;
	let mut distractors: Vec<String> = vec![];
	for _ in 0..NUM_DISTRACTORS {
		let random_song = songs.choose(&mut rand::thread_rng()).unwrap();
		let mut random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap();
		while are_close_enough(random_line, correct_answer) {
			random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap();
		}
		distractors.push(random_line.to_owned());
	}

	distractors
}



pub fn pick_random_guess(songs: &Vec<Song>) -> Question {
	let random_song = songs.choose(&mut rand::thread_rng()).unwrap();
	let mut random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap();
	while !is_acceptable_guess(random_line, &random_song.lines) {
		random_line = random_song.lines.choose(&mut rand::thread_rng()).unwrap();
	}
	let line_num = (&random_song.lines).into_iter().position(|r| r == random_line).unwrap();
	let answer = &random_song.lines[line_num + 1];

	Question {
		song_title: random_song.name.clone(),
		song_album: random_song.album.clone(),
		shown_line: random_line.clone(),
		answer: answer.clone(),
	}

}