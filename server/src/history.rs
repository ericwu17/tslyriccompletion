use std::collections::HashMap;
use rocket::time::format_description;
use serde::Deserialize;
use serde::Serialize;

use sqlx::{Pool, MySql, types::{time::PrimitiveDateTime, Json}};

#[derive(sqlx::FromRow, Debug)]
pub struct GameSchema {
	pub uuid: String,
	pub start_time: PrimitiveDateTime,
	pub songlist_sha: String,
	pub selected_songs: Json<HashMap<String, Vec<bool>>>,
	pub has_terminated: bool,
	pub terminal_score: Option<i32>,
	pub player_name: Option<String>,
	pub num_guesses: i32,
}

#[derive(Debug, Serialize)]

pub struct Game {
	pub uuid: String,
	pub start_time: String,
	pub songlist_sha: String,
	pub selected_songs: Vec<(String, String)>,
	pub has_terminated: bool,
	pub terminal_score: Option<i32>,
	pub player_name: Option<String>,
	pub num_guesses: i32,
}


#[derive(sqlx::FromRow, Debug)]
pub struct SonglistSchema {
	pub sha1sum: String,
	pub content: Json<Vec<(String, String)>>,
}

#[derive(Debug, Deserialize)]
pub struct Songlist {
	pub sha1sum: String,
	pub content: Vec<(String, String)>,
}

#[get("/history/all?<sort>&<search>&<limit>&<include_nameless>")]
pub async fn get_games(
	pool: &rocket::State<Pool<MySql>>,
	sort: Option<String>,
	search: Option<String>,
	limit: Option<usize>,
	include_nameless: Option<bool>,
) -> String {
	let sort = sort.unwrap_or("start_time".to_string());
	let search = format!("%{}%", search.unwrap_or("".to_string()));
	let limit = limit.unwrap_or(34);
	let include_nameless = include_nameless.unwrap_or(true);


	let songlists: Vec<SonglistSchema> = sqlx::query_as(
		"SELECT * from songlists")
		.fetch_all(pool.inner())
		.await.unwrap();

	let songlists: Vec<Songlist> = songlists.into_iter()
		.map(|songlist| Songlist{
			sha1sum: songlist.sha1sum,
			
			// We are serializing and then immediately deserializing because I can't figure out
			// how to convert the type from Json<Vec<(String, String)>> to Vec<(String, String)>
			content: serde_json::from_str(&serde_json::to_string(&songlist.content).unwrap()).unwrap(),
		}).collect();

		let sub_query = "select count(*) from guesses where game_uuid like uuid";
	
		let query = match sort.as_str() {
		"score" => {
			if include_nameless {
				format!("SELECT *, ({}) as num_guesses from games
				WHERE (player_name LIKE ? OR player_name IS NULL) AND has_terminated LIKE TRUE
				ORDER BY terminal_score DESC
				LIMIT ?", sub_query)
			} else {
				format!("SELECT *, ({}) as num_guesses from games
				WHERE (player_name LIKE ?) AND has_terminated LIKE TRUE
				ORDER BY terminal_score DESC
				LIMIT ?", sub_query)
			}
		}
		_ => {
			if include_nameless {
				format!("SELECT *, ({}) as num_guesses from games
				WHERE (player_name LIKE ? OR player_name IS NULL) AND has_terminated LIKE TRUE
				ORDER BY start_time DESC
				LIMIT ?", sub_query)
			} else {
				format!("SELECT *, ({}) as num_guesses from games
				WHERE (player_name LIKE ?) AND has_terminated LIKE TRUE
				ORDER BY start_time DESC
				LIMIT ?", sub_query)
			}
		}
	};

	let games: Vec<GameSchema> = sqlx::query_as(&query)
		.bind(search)
		.bind(limit as i32)
		.fetch_all(pool.inner())
		.await.unwrap();

	let games: Vec<Game> = games.into_iter()
		.map(|game| {
			let selected_songs =  serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap()).unwrap();
			let full_songlist = songlists.iter().filter(|s| s.sha1sum == game.songlist_sha).next().unwrap().content.clone();

			let selected_songs_desc = get_songs(full_songlist, selected_songs);

			let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();

			Game{
				uuid: game.uuid,
				start_time: game.start_time.format(&format).unwrap(),
				songlist_sha: game.songlist_sha,
				selected_songs: selected_songs_desc,
				has_terminated: game.has_terminated,
				terminal_score: game.terminal_score,
				player_name: game.player_name,
				num_guesses: game.num_guesses,
		}}).collect();

	serde_json::to_string(&games).unwrap()
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
	lifelines_used: Json<Vec<String>>,
	options: Json<Vec<String>>,
	submit_time: PrimitiveDateTime
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
}


impl Guess {
	pub fn from_schema(guess_schema: GuessSchema) -> Self {
		let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();

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
			lifelines_used: serde_json::from_str(&serde_json::to_string(&guess_schema.lifelines_used).unwrap()).unwrap(),
			options: serde_json::from_str(&serde_json::to_string(&guess_schema.options).unwrap()).unwrap(),
			submit_time: guess_schema.submit_time.format(&format).unwrap(),
		}
	}
}

#[derive(Serialize)]
struct GameWithGuesses {
	game: Game,
	guesses: Vec<Guess>,
}

fn get_songs (full_songlist: Vec<(String, String)>, selected_songs: HashMap<String, Vec<bool>>) -> Vec<(String, String)> {
	// full_songlist represents the list of all possible songs at the time of the selection.
	// selectedSongs is a hashmap where each key is an album, and the k-th boolean in the vector indicates whether
	// the k-th song of the album was included. (true means included)

	let mut songs: Vec<(String, String)> = Vec::new();

	let mut album_order = full_songlist.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
	album_order.dedup();  // remove duplicates since full_songlist contains one entry for each song but we only want a list of albums.


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




#[get("/history/game?<id>")]
pub async fn get_game(
	pool: &rocket::State<Pool<MySql>>,
	id: String,
) -> String {

	let songlists: Vec<SonglistSchema> = sqlx::query_as(
		"SELECT * from songlists")
		.fetch_all(pool.inner())
		.await.unwrap();

	let songlists: Vec<Songlist> = songlists.into_iter()
		.map(|songlist| Songlist{
			sha1sum: songlist.sha1sum,
			
			// We are serializing and then immediately deserializing because I can't figure out
			// how to convert the type from Json<Vec<(String, String)>> to Vec<(String, String)>
			content: serde_json::from_str(&serde_json::to_string(&songlist.content).unwrap()).unwrap(),
		}).collect();

	let game: GameSchema = sqlx::query_as(
		"SELECT *, (select count(*) from guesses where game_uuid like uuid) as num_guesses from games
		WHERE uuid LIKE ?")
		.bind(id.clone())
		.fetch_one(pool.inner())
		.await.unwrap();
	let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]Z").unwrap();
	
	let selected_songs =  serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap()).unwrap();
	let full_songlist = songlists.iter().filter(|s| s.sha1sum == game.songlist_sha).next().unwrap().content.clone();
	let selected_songs_desc = get_songs(full_songlist, selected_songs);


	let guesses: Vec<GuessSchema> = sqlx::query_as(
		"SELECT * from guesses
		WHERE game_uuid LIKE ?"
	).bind(id.clone())
		.fetch_all(pool.inner())
		.await.unwrap();
	
	let guesses: Vec<Guess> = guesses.into_iter().map(Guess::from_schema).collect();

	let game = Game{
		uuid: game.uuid,
		start_time: game.start_time.format(&format).unwrap(),
		songlist_sha: game.songlist_sha,
		selected_songs: selected_songs_desc,
		has_terminated: game.has_terminated,
		terminal_score: game.terminal_score,
		player_name: game.player_name,
		num_guesses: guesses.len() as i32,
	};

	serde_json::to_string(&GameWithGuesses{game, guesses}).unwrap()
}
