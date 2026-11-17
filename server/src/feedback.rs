use crate::rss::{RecentVotesCache, VoteEvent};
use chrono::prelude::*;
use rocket::serde::json::Json;
use serde::Deserialize;
use sqlx::{MySql, Pool};
use std::sync::{Arc, Mutex};

/// API endpoint to upvote a line
#[get("/feedback/upvote_line?<album>&<song_name>&<line>")]
pub async fn upvote_line(
    album: &str,
    song_name: &str,
    line: &str,
    pool: &rocket::State<Pool<MySql>>,
    vote_cache: &rocket::State<Arc<Mutex<RecentVotesCache>>>,
) -> String {
    record_vote(album, song_name, line, true, pool, vote_cache).await;
    "".to_owned()
}

/// API endpoint to downvote a line
#[get("/feedback/downvote_line?<album>&<song_name>&<line>")]
pub async fn downvote_line(
    album: &str,
    song_name: &str,
    line: &str,
    pool: &rocket::State<Pool<MySql>>,
    vote_cache: &rocket::State<Arc<Mutex<RecentVotesCache>>>,
) -> String {
    record_vote(album, song_name, line, false, pool, vote_cache).await;
    "".to_owned()
}

/// Records a vote in the database and in the recent-votes cache (used by the votes RSS feed).
async fn record_vote(
    album: &str,
    song_name: &str,
    line: &str,
    is_upvote: bool,
    pool: &Pool<MySql>,
    vote_cache: &Mutex<RecentVotesCache>,
) {
    let _ = sqlx::query("INSERT IGNORE INTO votes (album, song_name, lyric, num_upvotes, num_downvotes) VALUES (?, ?, ?, 0, 0);")
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool)
        .await;

    let update_query = if is_upvote {
        "UPDATE votes SET num_upvotes = num_upvotes + 1 WHERE album LIKE ? AND song_name LIKE ? AND lyric LIKE ?;"
    } else {
        "UPDATE votes SET num_downvotes = num_downvotes + 1 WHERE album LIKE ? AND song_name LIKE ? AND lyric LIKE ?;"
    };
    let _ = sqlx::query(update_query)
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool)
        .await;

    vote_cache.lock().unwrap().add(VoteEvent {
        time: Utc::now(),
        album: album.to_string(),
        song_name: song_name.to_string(),
        lyric: line.to_string(),
        is_upvote,
    });
}

#[derive(Deserialize, Debug)]
pub struct Feedback {
    album: String,
    song: String,
    lyric: String,
    message: String,
    contact: String,
}

/// API endpoint for general feedback
#[post(
    "/feedback/general",
    format = "application/json",
    data = "<feedback_data>"
)]
pub async fn get_feedback(
    feedback_data: Json<Feedback>,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let feedback_data = feedback_data.into_inner();
    dbg!(&feedback_data);
    let Feedback {
        album,
        song,
        lyric,
        message,
        contact,
    } = feedback_data;

    let _ = sqlx::query("INSERT INTO feedback (time, album, song_name, lyric, message, contact) VALUES (NOW(), ?, ?, ?, ?, ?)")
        .bind(album)
        .bind(song)
        .bind(lyric)
        .bind(message)
        .bind(contact)
        .fetch_all(pool.inner())
        .await;

    "".to_owned()
}
