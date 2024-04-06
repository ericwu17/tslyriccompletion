use atom_syndication::{Content, Entry, Feed};
use chrono::prelude::*;
use rocket::time::format_description;
use sqlx::{types::time::PrimitiveDateTime, MySql, Pool};
#[derive(sqlx::FromRow, Debug)]
pub struct GameSchema {
    pub time: PrimitiveDateTime,
    pub album: String,
    pub song_name: String,
    pub lyric: String,
    pub message: String,
    pub contact: String,
}

/// API endpoint to get a RSS feed of the most recent feedback
/// Returns the (up to) 20 most recent feedbacks by reading from the database and constructing the RSS feed
/// This endpoint is for my personal use, to see what people have been saying.
#[get("/feedback/get_recent_feedback_rss")]
pub async fn get_recent_feedback_rss(pool: &rocket::State<Pool<MySql>>) -> String {
    let recent_feedbacks: Vec<GameSchema> =
        sqlx::query_as("SELECT * FROM feedback ORDER BY time DESC LIMIT 20;")
            .fetch_all(pool.inner())
            .await
            .unwrap();

    let mut feed = Feed::default();
    feed.set_title("TSLC Feedback");
    feed.set_updated(Utc::now());

    let mut feed_entries = Vec::new();
    for f in recent_feedbacks {
        let mut entry = Entry::default();
        entry.set_title("New Feedback");
        entry.set_updated(Utc::now());

        let time_format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();
        let feedback_time = f.time.format(&time_format).unwrap();

        let mut content = Content::default();
        content.set_content_type("html".to_string());
        content.set_value(format!(
            "<p>Time: {}</p><p>Album: {}</p><p>Song: {}</p><p>Lyric: {}</p><p>Message:{}</p><p>Contact: {}</p>",
            feedback_time, f.album, f.song_name, f.lyric, f.message, f.contact,
        ));

        entry.set_content(content);
        feed_entries.push(entry);
    }
    feed.set_entries(feed_entries);
    feed.to_string()
}
