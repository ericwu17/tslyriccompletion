pub mod loader;
pub mod song;
pub mod guess_generating;
pub mod game;
pub mod diff;
pub mod lifelines;

use std::collections::HashMap;
use crate::song::Song;
use crate::loader::load_songs_from_files;

use game::{GameState,
	init_game,
	game_lifelines,
	reduce_multiple_choice,
	next_question,
	take_guess,
};
use rocket::State;
use std::sync::{Arc, Mutex};


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


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	std::env::set_var("RUST_BACKTRACE", "1");
	std::env::set_var("DATABASE_URL", "mysql://localhost:3306/mydb");
	let songs: Vec<Song> = load_songs_from_files();
	let my_hashmap: HashMap<String, GameState> = HashMap::new();
	let game_state = Arc::new(Mutex::new(my_hashmap));

	let database_url = "mysql://localhost:3306/mydb";
	println!("Connecting to MySql Database...");
	let pool = sqlx::MySqlPool::connect(database_url)
			.await
			.expect("Failed to connect to database");
	println!("Connection established!");

	let rocket = rocket::build()
		.manage(game_state)
		.manage(songs)
		.manage(pool)
		.mount("/", routes![index])
		.mount("/", routes![get_song_list])
		.mount("/", routes![get_song])
		.mount("/", routes![init_game])
		.mount("/", routes![game_lifelines])
		.mount("/", routes![reduce_multiple_choice])
		.mount("/", routes![next_question])
		.mount("/", routes![take_guess]).ignite().await?;
	
	let _ = rocket.launch().await?;

	Ok(())
}
