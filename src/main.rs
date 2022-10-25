pub mod loader;
pub mod song;
pub mod guess_generating;
pub mod game;
pub mod diff;
pub mod lifelines;

use std::collections::HashMap;
use crate::song::Song;
use crate::loader::load_songs_from_files;

use rocket::State;


struct SongList {
    songs: Vec<Song>
}
unsafe impl Send for SongList {}
unsafe impl Sync for SongList {}

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/songs")]
fn get_song_list(songs: &State<SongList>) -> String {
	let songs = &songs.songs;

	let mut s: HashMap<String, Vec<String>> = HashMap::new();
	for song in songs {
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
fn get_song(songs: &State<SongList>, album: &str, name: &str) -> String {
	let songs = &songs.songs;

	for song in songs {
		if song.album == album && song.name == name {
			return serde_json::to_string(&song).unwrap()
		}
	}

	"{}".to_string()
}


#[launch]
fn rocket() -> _{
	let songs: Vec<Song> = load_songs_from_files();
	


	rocket::build()
		.manage(SongList { songs })
		.mount("/", routes![index])
		.mount("/", routes![get_song_list])
		.mount("/", routes![get_song])
	// run_game_loop();


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
}
