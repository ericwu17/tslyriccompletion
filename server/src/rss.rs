use atom_syndication::{Content, Entry, Feed};
use chrono::prelude::*;
use rocket::time::format_description;
use sqlx::{types::time::PrimitiveDateTime, MySql, Pool};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

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

pub struct VoteEvent {
    pub time: DateTime<Utc>,
    pub album: String,
    pub song_name: String,
    pub lyric: String,
    pub is_upvote: bool,
}

const RECENT_VOTES_CACHE_SIZE: usize = 20;
pub struct RecentVotesCache {
    store: VecDeque<VoteEvent>,
}

impl RecentVotesCache {
    pub fn new() -> Self {
        RecentVotesCache {
            store: VecDeque::with_capacity(RECENT_VOTES_CACHE_SIZE),
        }
    }
    pub fn add(&mut self, new_vote: VoteEvent) {
        while self.store.len() >= RECENT_VOTES_CACHE_SIZE {
            self.store.pop_back();
        }
        self.store.push_front(new_vote);
    }
}

impl Default for RecentVotesCache {
    fn default() -> Self {
        Self::new()
    }
}

/// API endpoint to get a RSS feed of the most up and down votes
/// Returns the (up to) 20 most recent vote events
/// Note that this endpoint reads from the RecentVotesCache, which will be cleared whenever
/// the server restarts.
#[get("/feedback/get_recent_votes_rss")]
pub async fn get_recent_votes_rss(
    vote_cache: &rocket::State<Arc<Mutex<RecentVotesCache>>>,
) -> String {
    let guard = vote_cache.lock().unwrap();

    let mut feed = Feed::default();
    feed.set_title("TSLC Votes");
    feed.set_updated(Utc::now());

    let mut feed_entries = Vec::new();
    for vote_event in guard.store.iter() {
        let mut entry = Entry::default();
        if vote_event.is_upvote {
            entry.set_title("New Upvote");
        } else {
            entry.set_title("New Downvote");
        }
        entry.set_updated(vote_event.time);

        let mut content = Content::default();
        content.set_content_type("html".to_string());
        content.set_value(format!(
            r#"<p>Album: {}</p><p>Song: {}</p><p>Lyric: {}</p>"#,
            vote_event.album, vote_event.song_name, vote_event.lyric,
        ));

        entry.set_content(content);
        feed_entries.push(entry);
    }

    feed.set_entries(feed_entries);
    feed.to_string()
}
