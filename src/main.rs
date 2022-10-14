mod loader;

use std::error::Error;
use loader::load_songs_from_files;


#[derive(Debug)]
pub struct Song {
	pub album: String,
	pub name: String,
	pub lyrics_raw: String,
}


fn main() -> Result<(), Box<dyn Error>> {
	let songs: Vec<Song> = load_songs_from_files();
	for song in &songs {
		println!("{}, {}", song.name, song.album);
	}
	println!("{}", songs[0].lyrics_raw);

	Ok(())
}