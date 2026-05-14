//! Allows users to view guess details and score summaries of past games.

use rocket::time::format_description;
use serde::Deserialize;
use serde::Serialize;
use sqlx::{
    types::{time::PrimitiveDateTime, Json as SqlxJson},
    MySql, Pool,
};
use std::collections::HashMap;
use rocket::http::Status;
use rocket::serde::json::Json;

pub mod line_history;

#[derive(sqlx::FromRow, Debug)]
pub struct GameSchema {
    pub uuid: String,
    pub start_time: PrimitiveDateTime,
    pub songlist_id: i32,
    pub selected_songs: SqlxJson<HashMap<String, Vec<bool>>>,
    pub has_terminated: bool,
    pub terminal_score: Option<i32>,
    pub player_name: Option<String>,
    pub num_guesses: i32,
    pub user_id: Option<i32>,
    pub username: Option<String>,
}

/// Represents the summary of a past game.
#[derive(Debug, Serialize)]
pub struct Game {
    pub uuid: String,
    pub start_time: String,
    pub songlist_id: i32,
    pub selected_songs: Vec<(String, String)>,
    pub has_terminated: bool,
    pub terminal_score: Option<i32>,
    pub player_name: Option<String>,
    pub username: Option<String>,
    pub num_guesses: i32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct SonglistSchema {
    pub id: i32,
    pub sha1sum: String,
    pub content: SqlxJson<Vec<(String, String)>>,
}

/// Represents a list of songs available at a particular point in time.
/// Any game will draw a subset of songs from a particular `Songlist`.
/// For example, there's a songlist that contains every album released before Midnights.
#[derive(Debug, Deserialize)]
pub struct Songlist {
    /// `id` is used in a game to indicate which `SongList` is used
    pub id: i32,
    pub sha1sum: String,
    pub content: Vec<(String, String)>,
}

/// API endpoint to get all past games, with various filtering options.
/// Results are paginated.
#[get("/history/all?<sort>&<search>&<limit>&<include_nameless>&<page_num>")]
pub async fn get_games(
    pool: &rocket::State<Pool<MySql>>,
    sort: Option<String>,
    search: Option<String>,
    page_num: Option<usize>,
    limit: Option<usize>,
    include_nameless: Option<bool>,
) -> String {
    let sort = sort.unwrap_or_else(|| "start_time".to_string());
    let search = format!("%{}%", search.unwrap_or_default());
    let limit = limit.unwrap_or(20); // Default limit is 20 results per page
    let page_num = page_num.map_or(1, |num| if num > 0 { num } else { 1 });
    let include_nameless = include_nameless.unwrap_or(true);

    let query_offset = (page_num - 1) * limit;

    let songlists: Vec<SonglistSchema> = sqlx::query_as("SELECT * from songlists")
        .fetch_all(pool.inner())
        .await
        .unwrap();

    let songlists: Vec<Songlist> = songlists
        .into_iter()
        .map(|songlist| Songlist {
            id: songlist.id,
            sha1sum: songlist.sha1sum,
            content: songlist.content.as_ref().clone(),
        })
        .collect();

    let order_by_clause = match sort.as_str() {
        "score" => "ORDER BY terminal_score DESC",
        _ => "ORDER BY start_time DESC",
    };

    let where_clause = if include_nameless {
        "WHERE (player_name LIKE ? OR username LIKE ? OR (player_name IS NULL AND username IS NULL)) AND has_terminated LIKE TRUE"
    } else {
        "WHERE (player_name LIKE ? OR username LIKE ?) AND has_terminated LIKE TRUE"
    };

    let query = format!(
        "SELECT *, users.username from games
        LEFT JOIN users ON games.user_id = users.user_id
        {}
        {}
        LIMIT ? OFFSET ?",
        where_clause, order_by_clause
    );

    let games: Vec<GameSchema> = sqlx::query_as(&query)
        .bind(&search)
        .bind(&search)
        .bind(limit as i32)
        .bind(query_offset as i32)
        .fetch_all(pool.inner())
        .await
        .unwrap();

    let games: Vec<Game> = games
        .into_iter()
        .map(|game| {
            let selected_songs =
                serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap())
                    .unwrap();
            let full_songlist = songlists
                .iter()
                .find(|s| s.id == game.songlist_id)
                .unwrap()
                .content
                .clone();

            let selected_songs_desc = get_songs(full_songlist, selected_songs);

            let format =
                format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z")
                    .unwrap();

            Game {
                uuid: game.uuid,
                start_time: game.start_time.format(&format).unwrap(),
                songlist_id: game.songlist_id,
                selected_songs: selected_songs_desc,
                has_terminated: game.has_terminated,
                terminal_score: game.terminal_score,
                player_name: game.player_name,
                num_guesses: game.num_guesses,
                username: game.username,
            }
        })
        .collect();

    serde_json::to_string(&games).unwrap()
}

/// API endpoint to get games for a user by username (public, no auth required).
/// Results are paginated.
#[get("/users/<username>/games?<page_num>&<limit>")]
pub async fn get_user_games_by_username(
    pool: &rocket::State<Pool<MySql>>,
    username: String,
    page_num: Option<usize>,
    limit: Option<usize>,
) -> Result<String, Status> {
    // Resolve username to user_id
    let user_query: Option<(i32,)> = sqlx::query_as(
        "SELECT user_id FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    let (user_id,) = user_query.ok_or(Status::NotFound)?;

    let limit = limit.unwrap_or(20);
    let page_num = page_num.map_or(1, |num| if num > 0 { num } else { 1 });
    let query_offset = (page_num - 1) * limit;

    let songlists: Vec<SonglistSchema> = sqlx::query_as("SELECT * from songlists")
        .fetch_all(pool.inner())
        .await
        .map_err(|_| Status::InternalServerError)?;

    let songlists: Vec<Songlist> = songlists
        .into_iter()
        .map(|songlist| Songlist {
            id: songlist.id,
            sha1sum: songlist.sha1sum,
            content: songlist.content.as_ref().clone(),
        })
        .collect();

    let query = format!(
        "SELECT *, users.username from games
        LEFT JOIN users ON games.user_id = users.user_id
        WHERE games.user_id = ? AND has_terminated = TRUE
        ORDER BY start_time DESC
        LIMIT ? OFFSET ?"
    );

    let games: Vec<GameSchema> = sqlx::query_as(&query)
        .bind(user_id)
        .bind(limit as i32)
        .bind(query_offset as i32)
        .fetch_all(pool.inner())
        .await
        .map_err(|_| Status::InternalServerError)?;

    let games: Vec<Game> = games
        .into_iter()
        .map(|game| {
            let selected_songs =
                serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap())
                    .unwrap();
            let full_songlist = songlists
                .iter()
                .find(|s| s.id == game.songlist_id)
                .unwrap()
                .content
                .clone();

            let selected_songs_desc = get_songs(full_songlist, selected_songs);

            let format =
                format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z")
                    .unwrap();

            Game {
                uuid: game.uuid,
                start_time: game.start_time.format(&format).unwrap(),
                songlist_id: game.songlist_id,
                selected_songs: selected_songs_desc,
                has_terminated: game.has_terminated,
                terminal_score: game.terminal_score,
                player_name: game.player_name,
                num_guesses: game.num_guesses,
                username: game.username,
            }
        })
        .collect();

    Ok(serde_json::to_string(&games).unwrap())
}

#[derive(sqlx::FromRow, Debug)]
struct UserProfileSchema {
    username: String,
    created_at: PrimitiveDateTime,
    games_played: i64,
    guesses_made: i64,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub username: String,
    pub created_at: String,
    pub games_played: i64,
    pub guesses_made: i64,
}

#[get("/users/<username>/profile")]
pub async fn get_user_profile_by_username(
    pool: &rocket::State<Pool<MySql>>,
    username: String,
) -> Result<Json<UserProfile>, Status> {
    let user_profile: Option<UserProfileSchema> = sqlx::query_as(
        "SELECT users.username, users.created_at, COUNT(DISTINCT games.uuid) as games_played, COUNT(guesses.game_uuid) as guesses_made
        FROM users
        LEFT JOIN games ON games.user_id = users.user_id
        LEFT JOIN guesses ON guesses.game_uuid = games.uuid
        WHERE users.username = ?
        GROUP BY users.username, users.created_at",
    )
    .bind(&username)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    let user_profile = user_profile.ok_or(Status::NotFound)?;

    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();

    Ok(Json(UserProfile {
        username: user_profile.username,
        created_at: user_profile.created_at.format(&format).unwrap(),
        games_played: user_profile.games_played,
        guesses_made: user_profile.guesses_made,
    }))
}

#[derive(sqlx::FromRow, Debug)]
pub struct GuessSchema {
    game_uuid: String,
    order_num: i32,
    album: String,
    song_name: String,
    prompt: String,
    correct_answer: String,
    result: String,
    user_guess: String,
    points_earned: i32,
    lifeline_earned: Option<String>,
    lifelines_used: SqlxJson<Vec<String>>,
    options: SqlxJson<Vec<String>>,
    submit_time: PrimitiveDateTime,
}

/// Represents a single guess within a [`Game`]
#[derive(Serialize)]
pub struct Guess {
    game_uuid: String,
    order_num: i32,
    album: String,
    song_name: String,
    prompt: String,
    correct_answer: String,
    result: String,
    user_guess: String,
    points_earned: i32,
    lifeline_earned: Option<String>,
    lifelines_used: Vec<String>,
    options: Vec<String>,
    submit_time: String,
}

impl Guess {
    pub fn from_schema(guess_schema: GuessSchema) -> Self {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();

        Guess {
            game_uuid: guess_schema.game_uuid,
            order_num: guess_schema.order_num,
            album: guess_schema.album,
            song_name: guess_schema.song_name,
            prompt: guess_schema.prompt,
            correct_answer: guess_schema.correct_answer,
            result: guess_schema.result,
            user_guess: guess_schema.user_guess,
            points_earned: guess_schema.points_earned,
            lifeline_earned: guess_schema.lifeline_earned,
            lifelines_used: serde_json::from_str(
                &serde_json::to_string(&guess_schema.lifelines_used).unwrap(),
            )
            .unwrap(),
            options: serde_json::from_str(&serde_json::to_string(&guess_schema.options).unwrap())
                .unwrap(),
            submit_time: guess_schema.submit_time.format(&format).unwrap(),
        }
    }
}

/// A game with a list of guesses
#[derive(Serialize)]
struct GameWithGuesses {
    game: Game,
    guesses: Vec<Guess>,
}

/// Get selected selected songs in the form of (album, song_name) from a set of boolean arrays.
///
/// `full_songlist` represents the list of all possible songs at the time of the selection.
/// `selectedSongs` is a hashmap where each key is an album, and the k-th boolean in the vector indicates whether
/// the k-th song of the album was included (true means included).
fn get_songs(
    full_songlist: Vec<(String, String)>,
    selected_songs: HashMap<String, Vec<bool>>,
) -> Vec<(String, String)> {
    let mut songs: Vec<(String, String)> = Vec::new();

    let mut album_order = full_songlist
        .iter()
        .map(|x| x.0.clone())
        .collect::<Vec<String>>();
    album_order.dedup(); // remove duplicates since full_songlist contains one entry for each song but we only want a list of albums.

    for album in album_order {
        let inc_exc_list = selected_songs.get(&album).unwrap();

        let mut album_songs_iter = full_songlist.iter().filter(|s| s.0 == *album);
        for is_included in inc_exc_list {
            let curr_album_song = album_songs_iter.next().unwrap();
            if *is_included {
                songs.push(curr_album_song.clone());
            }
        }
    }

    songs
}

/// API endpoint for getting information about a game along with history of each guess
#[get("/history/game?<id>")]
pub async fn get_game(pool: &rocket::State<Pool<MySql>>, id: String) -> String {
    let songlists: Vec<SonglistSchema> = sqlx::query_as("SELECT * from songlists")
        .fetch_all(pool.inner())
        .await
        .unwrap();

    let songlists: Vec<Songlist> = songlists
        .into_iter()
        .map(|songlist| Songlist {
            id: songlist.id,
            sha1sum: songlist.sha1sum,
            content: songlist.content.as_ref().clone(),
        })
        .collect();

    let game: GameSchema = sqlx::query_as(
        "SELECT *, (select count(*) from guesses where game_uuid like uuid) as num_guesses, users.username from games
        LEFT JOIN users ON games.user_id = users.user_id
        WHERE uuid LIKE ?")
        .bind(id.clone())
        .fetch_one(pool.inner())
        .await.unwrap();
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();

    let selected_songs =
        serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap()).unwrap();
    let full_songlist = songlists
        .iter()
        .find(|s| s.id == game.songlist_id)
        .unwrap()
        .content
        .clone();
    let selected_songs_desc = get_songs(full_songlist, selected_songs);

    let guesses: Vec<GuessSchema> = sqlx::query_as(
        "SELECT * from guesses
        WHERE game_uuid LIKE ?",
    )
    .bind(id.clone())
    .fetch_all(pool.inner())
    .await
    .unwrap();

    let guesses: Vec<Guess> = guesses.into_iter().map(Guess::from_schema).collect();

    let game = Game {
        uuid: game.uuid,
        start_time: game.start_time.format(&format).unwrap(),
        songlist_id: game.songlist_id,
        selected_songs: selected_songs_desc,
        has_terminated: game.has_terminated,
        terminal_score: game.terminal_score,
        player_name: game.player_name,
        username: game.username,
        num_guesses: guesses.len() as i32,
    };

    serde_json::to_string(&GameWithGuesses { game, guesses }).unwrap()
}
