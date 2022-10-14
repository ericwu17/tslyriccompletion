mod loader;
mod song;

use std::error::Error;
use loader::load_songs_from_files;
use song::Song;



fn main() -> Result<(), Box<dyn Error>> {
	let songs: Vec<Song> = load_songs_from_files();
	for song in &songs {
		println!("{}, {}", song.name, song.album);
	}
	println!("{}", songs[5].lyrics_raw);
	println!("{:?}", songs[5].lines);

	Ok(())
}