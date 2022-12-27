pub mod loader;
pub mod song;
pub mod guess_generating;
pub mod game;
pub mod diff;
pub mod lifelines;
pub mod history;

use std::collections::HashMap;
use crate::song::Song;
use crate::loader::load_songs_from_files;
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;


use game::{GameState,
	init_game,
	game_lifelines,
	reduce_multiple_choice,
	next_question,
	take_guess,
	claim_game,
};
use history::{get_games, get_game};
use history::line_history::get_line;
use std::sync::{Arc, Mutex};
use song::{get_song, get_song_list};


#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	std::env::set_var("RUST_BACKTRACE", "1");
	dotenv().ok();
	let db_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set.");
	let db_pw = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set.");
	
	let songs: Vec<Song> = load_songs_from_files();
	let my_hashmap: HashMap<String, GameState> = HashMap::new();
	let game_state = Arc::new(Mutex::new(my_hashmap));

	let database_url = format!("mysql://{}:{}@localhost:3306/mydb", db_user, db_pw);
	println!("Connecting to MySql Database...");
	let pool = MySqlPoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
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
		.mount("/", routes![claim_game])
		.mount("/", routes![take_guess])
		.mount("/", routes![get_games])
		.mount("/", routes![get_game])
		.mount("/", routes![get_line])
		.ignite().await?;
	
	let _ = rocket.launch().await?;

	Ok(())
}
