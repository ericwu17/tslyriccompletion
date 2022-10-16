mod loader;
mod song;
mod guess_generating;
mod game;
mod diff;

use std::error::Error;
use game::{run_game_loop};

fn main() -> Result<(), Box<dyn Error>> {
	run_game_loop();


	// let songs: Vec<Song> = load_songs_from_files();
	// for song in songs {
	// 	for line in song.lines {
	// 		if line.is_exclamatory {
	// 			println!("[E]{}", line.text);
	// 		} else {
	// 			println!("{}", line.text);
	// 		}
	// 	}
	// 	println!("\n\n");
	// }


	Ok(())
}
