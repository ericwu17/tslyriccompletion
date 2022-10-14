mod loader;
mod song;
mod guess_generating;

use std::error::Error;
use loader::load_songs_from_files;
use song::Song;
use guess_generating::pick_random_guess;


fn main() -> Result<(), Box<dyn Error>> {
	let songs: Vec<Song> = load_songs_from_files();
	println!("{:?}", pick_random_guess(&songs));

	Ok(())
}