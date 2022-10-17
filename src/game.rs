use colored::Colorize;
use rand::Rng;
use std::io::{self, BufRead, Write};
use crate::loader::load_songs_from_files;
use crate::song::Song;
use crate::guess_generating::{pick_random_guess, optimal_truncated_dist, pick_distractors, Question};
use crate::diff::diff_greedy;
use crate::lifelines::{LifelineInventory, Lifeline};
use rand::prelude::SliceRandom;
use std::collections::HashSet;

const MAX_ACCEPTABLE_DIST: usize = 13;
const POINTS_FOR_PERFECT_MATCH: i32 = 26;

fn clear_screen() {
	// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	// print!("\x1B[2J\x1B[1;1H");
	// io::stdout().flush().unwrap();
	std::process::Command::new("clear").status().unwrap();

}

pub fn input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().unwrap();
	io::stdin()
		.lock()
		.lines()
		.next()
		.unwrap()
		.map(|x| x.trim_end().to_owned())
		.unwrap()
}

pub fn print_intro() {
	clear_screen();
	println!("{}

In this game, you will be shown a line from a Taylor Swift song. You will then be prompted to enter the following line. If your guess is close enough to the correct answer, you will score a certain number of points (and you earn a bonus if your guess is a perfect match).

If you do not know what the next line is, simply enter a question mark ('?') and you will be given 17 choices for the next line. Upon seeing the choices, enter a number 1-17, and you will score only 1 point for the correct answer.

After each round, press the enter key to continue, or enter '?' to view the full lyrics of the song.

The game ends as soon as you submit an incorrect answer.

In this game, there are 3 types of consumable lifelines that can be invoked by typing \"?s\", \"?t\", or \"?p\". ?s will skip the current question and move on to an entirely different one. ?t will show the album and song of the current question, and ?p will show you 3 consecutive lines and ask you to guess the next (giving you 2 additional lines of information). You may use ?l to view how many lifelines of each type you currently have and also see the keybindings for each lifeline.

You may only use lifelines before you choose to enter multiple-choice mode by pressing \"?\".

You will earn a lifeline for each perfect match, and you'll also have chance of getting a lifeline for each correct guess (the closer the match, the higher the chance of getting a lifeline).

Good luck! And have fun!", 

"Welcome to a Taylor Swift lyric guessing game!".blue().bold());
}

pub fn print_help() {
	println!("This is the help menu: Coming soon!");
}

fn print_with_flags(text: &str, flags: Vec<i32>) {
	if text.chars().count() != flags.len() {
		panic!("called print_with_flags with mismatched text and flag");
	}
	for i in 0..flags.len() {
		match flags[i] {
			1 => {
				print!("{}", text.chars().nth(i).unwrap().to_string().red().bold())
			},
			2 => {
				print!("{}", text.chars().nth(i).unwrap().to_string().yellow().bold())
			}
			_ => {
				print!("{}", text.chars().nth(i).unwrap())
			},
		}
	}
	println!("");
}

fn print_guess_with_answer(guess: &str, answer: &str, optimal_truncate_amt: i32) {
	let (_, diffs) = 
		diff_greedy(
			&guess.to_lowercase()[0..(guess.len()-optimal_truncate_amt as usize)], 
			&answer.to_lowercase(),
		).unwrap();
	
	let mut guess_flags = vec![0; guess.chars().count()];
	let mut ans_flags = vec![0; answer.chars().count()];
	for insertion in diffs.get("insert").unwrap() {
		for i in insertion.at..=insertion.to {
			ans_flags[i] = 1;
		}
	}
	for deletion in diffs.get("delete").unwrap() {
		for i in deletion.at..=deletion.to {
			guess_flags[i] = 1;
		}
	}
	for i in (guess_flags.len()-optimal_truncate_amt as usize)..guess_flags.len() {
		guess_flags[i] = 2;
	}

	print!("{}", "   Your Answer: ".blue().bold());
	print_with_flags(guess, guess_flags);
	print!("{}", "Correct Answer: ".blue().bold());
	print_with_flags(answer, ans_flags);
}


fn print_song(song: &Song, highlighted_line: &str) {
	println!("{}", format!("{} : {}", song.album, song.name).green().bold());
	for line in song.lyrics_raw.split("\n") {
		if line.trim() == highlighted_line {
			println!("{}", line.green().bold());
		} else {
			println!("{}", line);
		}
	}
	println!("{}", format!("{} : {}", song.album, song.name).green().bold());
}

fn take_user_multiple_choice_guess(answer: &str, choices: &Vec<String>) -> bool{
	// returns true if the user gets the correct answer
	for i in 0..choices.len(){
			println!("{}{} {}",
				(i+1).to_string().blue().bold(),
				")".blue().bold(),
				choices[i]
			);
	}
	let acceptable_inputs: Vec<String> = (1..=choices.len()).map(|x| x.to_string()).collect();
	let chosen_index: usize;
	loop {
		let guess = input(">>> ");
		if guess == "/" {
			return false;
		}
		if !acceptable_inputs.contains(&guess) {
			println!("That's not a valid choice! (You can no longer use lifelines after opting for multiple choice)")
		} else {
			chosen_index = guess.parse::<usize>().unwrap() - 1;
			break
		}

	}

	return choices[chosen_index] == answer;
}

pub fn is_on_right_track(guess: &str, answer: &str) -> bool {
	let (_, dist) = optimal_truncated_dist(answer, guess);
	dist <= MAX_ACCEPTABLE_DIST
}

fn print_previous_lines(question: &Question) {
	const PREV_LINES_TO_SHOW: usize = 2;

	let lines = question.song.lines.clone();
	let mut answer_position: usize = 0;
	for index in 0..lines.len() {
		if lines[index].text == question.shown_line {
			answer_position = index;
		}
	}
	if answer_position <= PREV_LINES_TO_SHOW {
		println!("{}", "This is the beginning of the song:".red().bold());
	}
	let beginning_index = std::cmp::max(answer_position as i32 - PREV_LINES_TO_SHOW as i32, 0);

	for index in (beginning_index as usize)..=answer_position {
		println!("{}", lines[index].text.blue().bold());
	}


}
pub fn take_selection_from_songs(songs: Vec<Song>) -> Vec<Song> {
	let mut albums: HashSet<&str> = HashSet::new();
	for song in &songs {
		albums.insert(song.album.as_str());
	}
	let mut albums: Vec<&str> = albums.into_iter().collect();
	albums.sort();
	clear_screen();
	let ordinals = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];


	for (index, album_name) in albums.iter().enumerate() {
		println!("{} {}", (ordinals[index].to_string() + ")").blue().bold(), album_name);
	}

	println!("Please make a selection as follows:\n\tEnter \"{}{}{}\" if you would like to play with only {}, {}, and {}.\n\tEnter \"!{}{}\" if you would like to play with all albums except {} and {}.", 
		ordinals[0], ordinals[2], ordinals[3], 
		albums[0], albums[2], albums[3], 
		ordinals[2], ordinals[1],
		albums[2], albums[1],
	);

	let res = input("Please make a selection: ");
	let mut selected_indices: Vec<usize> = Vec::new();
	for character in (&res).chars() {
		if let Some(index) = ordinals.iter().position(|&x| *x == character.to_string()) {
			if index < albums.len() {
				selected_indices.push(index);
			}
		}
	}
	let selected_albums:Vec<&str> = selected_indices.iter().map(|x| albums[*x]).collect();

	let mut included_albums: Vec<&str> = if res.starts_with("!") {
		albums.clone().into_iter().filter(|x| !selected_albums.contains(x)).collect()
	} else {
		selected_albums
	};
	if included_albums.len() == 0 {
		included_albums = albums.clone();
	}

	println!("You are about to play the game with the following albums:");
	for a in &included_albums {
		println!("\t{}", a.blue().bold());
	}
	input("Please press enter to confirm: ");

	songs.clone().into_iter().filter(|x| included_albums.contains(&x.album.as_str())).collect()
}


pub fn run_game_loop() {
	let mut score = 0;
	let mut lifeline_inv = LifelineInventory::new();
	let mut songs: Vec<Song> = load_songs_from_files();
	print_intro();

	if input("Press enter to begin (or type \"s\" to make an album selection). ") == "s" {
		songs = take_selection_from_songs(songs);
	}

	let mut question = pick_random_guess(&songs);
	clear_screen();
	println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
	'main_game: loop  {
		
		
		let dist: usize;

		let mut guess = input(">>> ");
		loop {
			if guess == "?" {
				println!("Reducing to a multiple choice challenge: ");
				let mut choices = pick_distractors(&question.answer, &songs);
				choices.push(question.answer.to_owned());
				choices.shuffle(&mut rand::thread_rng());

				let result = take_user_multiple_choice_guess(&question.answer, &choices);
				dist = if result { MAX_ACCEPTABLE_DIST } else { MAX_ACCEPTABLE_DIST + 1 };
				
				break;
			}

			else if guess == "?l" {
				println!("{}", lifeline_inv);
				continue 'main_game;
			}

			else if guess == "?h" {
				print_help();
				continue 'main_game;
			}

			else if guess == "?s" {
				if lifeline_inv.consume_lifeline(Lifeline::Skip) {
					print_song(&question.song, &question.shown_line);
					input("Press enter to continue: ");
					question = pick_random_guess(&songs);
					clear_screen();
					println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
					continue 'main_game;
				} else {
					println!("You do not have any of these lifelines left!");
					guess = input(">>> ");
				}
				
			}

			else if guess == "?t" {
				if lifeline_inv.consume_lifeline(Lifeline::ShowTitleAlbum) {
					println!("{}", format!("{} : {}", question.song.album, question.song.name).green().bold());
					guess = input(">>> ");
				} else {
					println!("You do not have any of these lifelines left!");
					guess = input(">>> ");
				}
			}

			else if guess == "?p" {
				if lifeline_inv.consume_lifeline(Lifeline::ShowPrevLines) {
					print_previous_lines(&question);
					guess = input(">>> ");
				} else {
					println!("You do not have any of these lifelines left!");
					guess = input(">>> ");
				}
			}

			else if guess.chars().count() < question.answer.chars().count() - 5 && guess != "/" && is_on_right_track(&guess, &question.answer) {
				println!("You're on the right track, but your guess was too short!");
				guess = input(">>> ");
			} else {
				let (truncate_amt, dist_after_truncation) = optimal_truncated_dist(&guess, &question.answer);
		
				print_guess_with_answer(&guess, &question.answer, truncate_amt);

				dist = dist_after_truncation;
				break;
			}
		}



		if dist > MAX_ACCEPTABLE_DIST {
			println!("That wasn't it! The game is over now, thanks for playing!");
			let response = input("Press enter to quit ('?' to show song, 'r' to restart): ");
			if response == "?" {
				print_song(&question.song, &question.shown_line);
				if input("Press enter to quit, r to restart:") == "r" {
					score = 0;
					lifeline_inv = LifelineInventory::new();
					question = pick_random_guess(&songs);
					clear_screen();
					println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
					continue;
				}
			} else if response == "r" {
				score = 0;
				lifeline_inv = LifelineInventory::new();
				question = pick_random_guess(&songs);
				clear_screen();
				println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
				continue;
			}
			std::process::exit(0);
		} else if dist != 0 {
			let points_earned = (MAX_ACCEPTABLE_DIST - dist + 1) as i32;
			println!("Correct! You scored {} points for your answer.", points_earned.to_string().green());

			let mut maybe_new_lifeline: Option<Lifeline> = None;

			if rand::thread_rng().gen_range(0..MAX_ACCEPTABLE_DIST) > dist {
				maybe_new_lifeline = Some(Lifeline::random_lifeline());
			}

			if let Some(new_lifeline) = maybe_new_lifeline {
				lifeline_inv.add_lifeline(&new_lifeline);
				println!("You also got a {}", new_lifeline);
			}
			println!("{}", lifeline_inv);
			score += points_earned;
		} else {
			let new_lifeline = Lifeline::random_lifeline();
			lifeline_inv.add_lifeline(&new_lifeline);
			println!("Yes! You scored {POINTS_FOR_PERFECT_MATCH} points for your {}, and got a {}", "perfect match".green().bold(), new_lifeline);
			println!("{}", lifeline_inv);
			score += POINTS_FOR_PERFECT_MATCH;
		}


		// println!("The correct answer was {} and the distance was {}, {}", question.answer, dist, truncate_amt);
		let response = input("Press enter to continue ('?' to show song): ");
		if response == "?" {
			print_song(&question.song, &question.shown_line);
			input("Press enter to continue: ");
		}

		question = pick_random_guess(&songs);
		clear_screen();
		println!("Your current score is {}. What line follows: \n\t{}", score.to_string().green(),question.shown_line.blue().bold());
	}
}
