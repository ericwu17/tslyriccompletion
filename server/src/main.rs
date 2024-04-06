pub mod diff;
pub mod feedback;
pub mod game;
pub mod guess_generating;
pub mod history;
pub mod lifelines;
pub mod loader_v2;
pub mod rss;
pub mod song;

use crate::rss::RecentVotesCache;
use crate::song::Song;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::collections::HashMap;

use feedback::{downvote_line, get_feedback, upvote_line};
use game::{
    claim_game, game_lifelines, init_game, next_question, reduce_multiple_choice, take_guess,
    GameState,
};
use history::line_history::get_line;
use history::{get_game, get_games};
use rss::{get_recent_feedback_rss, get_recent_votes_rss};
use song::{get_all_songlists, get_song, get_song_list, get_song_list_with_id};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

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

    let songs: Vec<Song> = loader_v2::load_songs_from_files();
    let my_hashmap: HashMap<String, GameState> = HashMap::new();
    let game_state = Arc::new(Mutex::new(my_hashmap));
    let votes_cache = Arc::new(Mutex::new(RecentVotesCache::new()));

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
        .manage(votes_cache)
        .mount("/", routes![index])
        .mount("/", routes![get_song_list])
        .mount("/", routes![get_song_list_with_id])
        .mount("/", routes![get_all_songlists])
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
        .mount("/", routes![upvote_line])
        .mount("/", routes![downvote_line])
        .mount("/", routes![get_feedback])
        .mount("/", routes![get_recent_feedback_rss])
        .mount("/", routes![get_recent_votes_rss])
        .ignite()
        .await?;

    let _ = rocket.launch().await?;

    Ok(())
}
