use colored::Colorize;
use std::io::{self, BufRead, Write};
use crate::loader::load_songs_from_files;
use crate::song::Song;
use crate::guess_generating::pick_random_guess;

fn clear_screen() {
	// print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	// print!("\x1B[2J\x1B[1;1H");
	// io::stdout().flush().unwrap();
	std::process::Command::new("clear").status().unwrap();

}

fn input(prompt: &str) -> String {
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

Good luck! And have fun!", 

"Welcome to a Taylor Swift lyric guessing game!".blue().bold());

	input("Press enter to begin.");
	clear_screen();
}


pub fn run_game_loop() {
	let songs: Vec<Song> = load_songs_from_files();
	print_intro();

	loop {
		println!("{:?}", pick_random_guess(&songs));
	}
}
