use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::default::Default;

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
}

#[derive(Debug, Serialize)]

pub struct Game {
	pub uuid: String,
	pub start_time: String,
	pub songlist_sha: String,
	pub selected_songs: SonglistChoiceDescription,
	pub has_terminated: bool,
	pub terminal_score: Option<i32>,
	pub player_name: Option<String>,
}

#[derive(Serialize, Debug)]

pub enum SongSet {
	Song (String, String),
	Album (String),
}

#[derive(Serialize, Debug)]
pub enum SonglistChoiceDescription {
	Exclude (Vec<SongSet>),
	Include (Vec<SongSet>),
}

impl Default for SonglistChoiceDescription {
	fn default() -> Self {
		SonglistChoiceDescription::Exclude(vec![])
	}
}

impl SonglistChoiceDescription {
	pub fn from_db(full_songlist: Vec<(String, String)>, selected_songs: HashMap<String, Vec<bool>>) -> Self{
		// full_songlist represents the list of all possible songs at the time of the selection.
		// selectedSongs is a hashmap where each key is an album, and the k-th boolean in the vector indicates whether
		// the k-th song of the album was included. (true means included)

		let mut num_inc = 0;
		let mut num_exc = 0;

		let mut album_order = full_songlist.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
		album_order.dedup();  // remove duplicates since full_songlist contains one entry for each song but we only want a list of albums.

		for (_, inc_exc_list) in &selected_songs {
			for v in inc_exc_list {
				if *v {
					num_inc += 1;
				} else {
					num_exc += 1;
				}
			}
		}

		// if num_inc > num_exc then we'll describe the songlist based on what's excluded.
		// otherwise we describe it based on what's included.
		if num_inc > num_exc {
			let mut exc_arr: Vec<SongSet> = vec![];

			for album in album_order {
				let inc_exc_list = selected_songs.get(&album).unwrap();
				if inc_exc_list.iter().all(|v| *v == false) {
					exc_arr.push(SongSet::Album(album.clone()));
					continue;
				}

				let mut album_songs_iter = full_songlist.iter().filter(|s| s.0 == *album);
				for v in inc_exc_list {
					let current_song = album_songs_iter.next().unwrap();
					if *v == false {
						exc_arr.push(SongSet::Song(current_song.0.clone(), current_song.1.clone()));
					}
				}
			}
			Self::Exclude(exc_arr)
		} else {
			let mut inc_arr: Vec<SongSet> = vec![];

			for album in album_order {
				let inc_exc_list = selected_songs.get(&album).unwrap();

				if inc_exc_list.iter().all(|v| *v == true) {
					inc_arr.push(SongSet::Album(album.clone()));
					continue;
				}

				let mut album_songs_iter = full_songlist.iter().filter(|s| s.0 == *album);
				for v in inc_exc_list {
					let current_song = album_songs_iter.next().unwrap();
					if *v == true {
						inc_arr.push(SongSet::Song(current_song.0.clone(), current_song.1.clone()));
					}
				}
			}
			Self::Include(inc_arr)
		}




	}
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

	
		let query = match sort.as_str() {
		"score" => {
			if include_nameless {
				"SELECT * from games
				WHERE player_name LIKE ? OR player_name IS NULL
				ORDER BY terminal_score DESC
				LIMIT ?"
			} else {
				"SELECT * from games
				WHERE player_name LIKE ?
				ORDER BY terminal_score DESC
				LIMIT ?"
			}
		}
		_ => {
			if include_nameless {
				"SELECT * from games
				WHERE player_name LIKE ? OR player_name IS NULL
				ORDER BY start_time DESC
				LIMIT ?"
			} else {
				"SELECT * from games
				WHERE player_name LIKE ?
				ORDER BY start_time DESC
				LIMIT ?"
			}
		}
	};

	let games: Vec<GameSchema> = sqlx::query_as(query)
		.bind(search)
		.bind(limit as i32)
		.fetch_all(pool.inner())
		.await.unwrap();

	let games: Vec<Game> = games.into_iter()
		.map(|game| {
			let selected_songs =  serde_json::from_str(&serde_json::to_string(&game.selected_songs).unwrap()).unwrap();
			let full_songlist = songlists.iter().filter(|s| s.sha1sum == game.songlist_sha).next().unwrap().content.clone();

			let selected_songs_desc = SonglistChoiceDescription::from_db(full_songlist, selected_songs);

			Game{
				uuid: game.uuid,
				start_time: game.start_time.to_string(),
				songlist_sha: game.songlist_sha,
				selected_songs: selected_songs_desc,
				has_terminated: game.has_terminated,
				terminal_score: game.terminal_score,
				player_name: game.player_name,
		}}).collect();

	serde_json::to_string(&games).unwrap()
}
