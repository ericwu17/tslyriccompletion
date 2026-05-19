pub mod auth;
pub mod diff;
pub mod feedback;
pub mod game;
pub mod guess_generating;
pub mod history;
pub mod leaderboard;
pub mod lifelines;
pub mod loader_v2;
pub mod rss;
pub mod song;
pub mod stats;

use crate::rss::RecentVotesCache;
use crate::song::Song;
use crate::stats::{get_stats, StatsResponse};
use chrono::{Datelike, Local};
use dotenv::dotenv;
use leaderboard::award_monthly_medals;
use rocket::tokio;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use sqlx::Pool;
use std::collections::HashMap;
use std::time::Duration;

use auth::login::login;
use auth::logout::logout;
use auth::password_reset::{request_password_reset, reset_password};
use auth::personal_details::get_personal_details;
use auth::signup::signup;
use auth::verify_email::{request_email_verification, verify_email};
use feedback::{downvote_line, get_feedback, upvote_line};
use game::{
    claim_game, game_lifelines, init_game, next_question, reduce_multiple_choice, take_guess,
    GameState,
};
use history::line_history::get_line;
use history::{get_game, get_games, get_user_games_by_username, get_user_profile_by_username};
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
    let stats_cache: Arc<Mutex<Option<StatsResponse>>> = Default::default();

    let database_url = format!("mysql://{}:{}@localhost:3306/mydb", db_user, db_pw);
    println!("Connecting to MySql Database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    println!("Connection established!");

    let pool_for_scheduler = pool.clone();
    tokio::spawn(async move {
        schedule_monthly_medal_award(pool_for_scheduler).await;
    });

    award_monthly_medals(&pool, 2026, 5)
        .await
        .expect("Failed to award monthly medals");

    let rocket = rocket::build()
        .manage(game_state)
        .manage(songs)
        .manage(pool)
        .manage(votes_cache)
        .manage(stats_cache)
        .mount("/", routes![index])
        .mount("/", routes![get_stats])
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
        .mount("/", routes![get_user_games_by_username])
        .mount("/", routes![get_user_profile_by_username])
        .mount("/", routes![upvote_line])
        .mount("/", routes![downvote_line])
        .mount("/", routes![get_feedback])
        .mount("/", routes![get_recent_feedback_rss])
        .mount("/", routes![get_recent_votes_rss])
        .mount("/", routes![signup])
        .mount("/", routes![login])
        .mount("/", routes![logout])
        .mount("/", routes![request_email_verification])
        .mount("/", routes![verify_email])
        .mount("/", routes![request_password_reset])
        .mount("/", routes![reset_password])
        .mount("/", routes![get_personal_details])
        .ignite()
        .await?;

    let _ = rocket.launch().await?;

    Ok(())
}

async fn schedule_monthly_medal_award(pool: Pool<MySql>) {
    // Track the last seen month+year and poll every 10 minutes.
    let mut last_month = Local::now().month();
    let mut last_year = Local::now().year();

    loop {
        tokio::time::sleep(Duration::from_secs(10 * 60)).await; // 10 minutes

        let now = Local::now();
        if now.month() != last_month || now.year() != last_year {
            // Month changed — award for the month that just finished.
            let (year_to_award, month_to_award) = if now.month() == 1 {
                (now.year() - 1, 12)
            } else {
                (now.year(), now.month() as i32 - 1)
            };
            let _ = award_monthly_medals(&pool, year_to_award, month_to_award).await;

            last_month = now.month();
            last_year = now.year();
        }
    }
}
