use rocket::time::format_description;
use serde::Serialize;
use sqlx::{
    types::{time::PrimitiveDateTime, Json},
    MySql, Pool,
};

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
    lifelines_used: Json<Vec<String>>,
    options: Json<Vec<String>>,
    submit_time: PrimitiveDateTime,
    player_name: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct VotesSchema {
    num_upvotes: i32,
    num_downvotes: i32,
}

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
    player_name: Option<String>,
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
            player_name: guess_schema.player_name,
        }
    }
}

#[derive(Serialize)]
pub struct LineResult {
    guesses: Vec<Guess>,
    num_upvotes: i32,
    num_downvotes: i32,
}

#[get("/history/line?<album>&<song>&<prompt>")]
pub async fn get_line(
    pool: &rocket::State<Pool<MySql>>,
    album: &str,
    song: &str,
    prompt: &str,
) -> String {
    let guesses: Vec<GuessSchema> = sqlx::query_as(
        "SELECT guesses.*, games.player_name from guesses
        INNER JOIN games ON guesses.game_uuid=games.uuid
        WHERE
        album LIKE ?
        AND song_name LIKE ?
        AND prompt LIKE ?
        ORDER BY submit_time DESC
        ",
    )
    .bind(album)
    .bind(song)
    .bind(prompt)
    .fetch_all(pool.inner())
    .await
    .unwrap();

    let guesses: Vec<Guess> = guesses.into_iter().map(Guess::from_schema).collect();

    let votes: Vec<VotesSchema> = sqlx::query_as(
        "SELECT * FROM votes WHERE album LIKE ? AND song_name LIKE ? AND lyric LIKE ?;",
    )
    .bind(album)
    .bind(song)
    .bind(prompt)
    .fetch_all(pool.inner())
    .await
    .unwrap();

    let vote_schema = votes.first();

    let num_upvotes;
    let num_downvotes;
    match vote_schema {
        Some(v) => {
            num_upvotes = v.num_upvotes;
            num_downvotes = v.num_downvotes;
        }
        None => {
            num_upvotes = 0;
            num_downvotes = 0;
        }
    };

    let line_result = LineResult {
        guesses,
        num_downvotes,
        num_upvotes,
    };

    serde_json::to_string(&line_result).unwrap()
}
