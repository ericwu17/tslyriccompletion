use std::error::Error;
use include_dir::{include_dir, Dir};


static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/taylor/data-raw/lyrics");

static EXCLUDED_ALBUMS: [&str;8] = [
	"91_beautiful-eyes",
	"98_non-album",
	"99_features",
	"90_the-taylor-swift-holiday-collection",

	"02_fearless",
	"02a_fearless-platinum-edition",
	"04_red",
	"04a_red-deluxe-edition",
];

static EXCLUDED_SONGS: [&str; 3] = [
	"15_teardrops-on-my-guitar-popv",
	"20_state-of-grace-acousticv-tv",
	"16_forever-n-always-pianov-tv",
];


#[derive(Debug)]
pub struct Song {
	pub album: String,
	pub name: String,
	pub lyrics: String,
}


fn main() -> Result<(), Box<dyn Error>> {

	let songs: Vec<Song> = load_songs_from_files();
	for song in songs {
		println!("{}, {}", song.name, song.album);
	}

	Ok(())
}


fn capitalize_first_letter(s: &str) -> String {
	if s.len() == 0 {
		return "".to_owned()
	}
    s[0..1].to_uppercase() + &s[1..]
}
fn process_album_name(name: &str) -> String {
	let mut name = name;
	let suffixes_to_remove = ["taylors-version", "deluxe-edition", "deluxe"];

	for suffix in suffixes_to_remove {
		name = name.trim_end_matches(suffix);
	}
	name.split("-")
		.map(|x| capitalize_first_letter(x))
		.collect::<Vec<String>>()
		.join(" ")
}

fn process_song_name(name: &str) -> String {
	let mut name = name;
	let suffixes_to_remove = ["-tv", "-10mv-tv-ftv", "-tv-ftv"];

	for suffix in suffixes_to_remove {
		name = name.trim_end_matches(suffix);
	}
	name.split("-")
		.map(|x| capitalize_first_letter(x))
		.collect::<Vec<String>>()
		.join(" ")
}


fn load_songs_from_files() -> Vec<Song> {
	let mut songs: Vec<Song> = Vec::new();


	let mut curr_album_name: String = "".to_owned();
	let mut curr_index: String = "".to_owned();
	for album_dir in PROJECT_DIR.dirs() {
		let album_name_full = album_dir.path().file_name().unwrap().to_str().unwrap();
		if EXCLUDED_ALBUMS.contains(&album_name_full) {
			continue;
		}
		let split = album_name_full.split("_").collect::<Vec<&str>>();
		let (index_str, name) = (split[0], split[1]);
		let index: String = index_str.chars().filter(|c| c.is_digit(10)).collect();
		if index != curr_index {
			curr_index = index;
			curr_album_name = name.to_owned();
		}

		for song_file in album_dir.files() {
			let song_name_full = song_file.path().file_stem().unwrap().to_str().unwrap();
			if EXCLUDED_SONGS.contains(&song_name_full) {
				continue;
			}
			let split = song_name_full.split("_").collect::<Vec<&str>>();
			let song_name = split[1];

			let song_lyrics = song_file.contents_utf8().unwrap();
			songs.push(Song { album: process_album_name(&curr_album_name), name: process_song_name(song_name), lyrics: song_lyrics.to_owned() });
		}
	}
	songs
}