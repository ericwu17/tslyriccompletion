pub mod loader;
pub mod song;
pub mod guess_generating;
pub mod game;
pub mod diff;
pub mod lifelines;

use std::{collections::HashMap, sync::atomic::AtomicUsize};
use crate::song::Song;
use crate::loader::load_songs_from_files;

use game::{init_game, game_lifelines, reduce_multiple_choice, GameState};
use rocket::State;
use std::sync::{Arc, Mutex};


pub static NEXT_GAME_ID: AtomicUsize = AtomicUsize::new(1);


#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/songs")]
fn get_song_list(songs: &State<Vec<Song>>) -> String {
	let mut s: HashMap<String, Vec<String>> = HashMap::new();
	for song in songs.iter() {
		if let Some(v) = s.get(&song.album) {
			let mut v = v.clone();
			v.push(song.name.clone());
			s.insert(song.album.clone(), v);
		} else {
			s.insert(song.album.clone(), vec![song.name.clone()]);
		}
	}
	println!("{:?}", songs);

	serde_json::to_string(&s).unwrap()
}

#[get("/songs/<album>/<name>")]
fn get_song(songs: &State<Vec<Song>>, album: &str, name: &str) -> String {
	for song in songs.iter() {
		if song.album == album && song.name == name {
			return serde_json::to_string(&song).unwrap()
		}
	}

	"{}".to_string()
}


#[launch]
fn rocket() -> _{
	let songs: Vec<Song> = load_songs_from_files();
	let my_hashmap: HashMap<usize, GameState> = HashMap::new();
	let game_state = Arc::new(Mutex::new(my_hashmap));

	rocket::build()
		.manage(game_state)
		.manage(songs)
		.mount("/", routes![index])
		.mount("/", routes![get_song_list])
		.mount("/", routes![get_song])
		.mount("/", routes![init_game])
		.mount("/", routes![game_lifelines])
		.mount("/", routes![reduce_multiple_choice])
}
