use rocket::serde::json::Json;
use serde::Deserialize;
use sqlx::{MySql, Pool};

/// API endpoint to upvote a line
#[get("/feedback/upvote_line?<album>&<song_name>&<line>")]
pub async fn upvote_line(
    album: &str,
    song_name: &str,
    line: &str,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let _ = sqlx::query("INSERT IGNORE INTO votes (album, song_name, lyric, num_upvotes, num_downvotes) VALUES (?, ?, ?, 0, 0);")
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool.inner())
        .await;
    let _ = sqlx::query("UPDATE votes SET num_upvotes = num_upvotes + 1 WHERE album = ? AND song_name = ? AND lyric = ?;")
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool.inner())
        .await;

    "".to_owned()
}

/// API endpoint to downvote a line
#[get("/feedback/downvote_line?<album>&<song_name>&<line>")]
pub async fn downvote_line(
    album: &str,
    song_name: &str,
    line: &str,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let _ = sqlx::query("INSERT IGNORE INTO votes (album, song_name, lyric, num_upvotes, num_downvotes) VALUES (?, ?, ?, 0, 0);")
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool.inner())
        .await;
    let _ = sqlx::query("UPDATE votes SET num_downvotes = num_downvotes + 1 WHERE album = ? AND song_name = ? AND lyric = ?;")
        .bind(album)
        .bind(song_name)
        .bind(line)
        .fetch_all(pool.inner())
        .await;

    "".to_owned()
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
