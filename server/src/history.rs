use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;



use sqlx::{Pool, MySql, types::{time::PrimitiveDateTime, Json}};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct GameSchema {
	pub uuid: String,
	#[serde(serialize_with = "serialize_datetime")]
	pub start_time: PrimitiveDateTime,
	pub songlist_sha: String,
	pub selected_songs: Json<HashMap<String, Vec<bool>>>,
	pub has_terminated: bool,
	pub terminal_score: Option<i32>,
	pub player_name: Option<String>,
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


fn serialize_datetime<S>(x: &PrimitiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	s.serialize_str(&x.to_string())
}


#[get("/history/all")]
pub async fn get_games(pool: &rocket::State<Pool<MySql>>) -> String {
	let songs: Vec<SonglistSchema> = sqlx::query_as(
		"SELECT * from songlists")
		.fetch_all(pool.inner())
		.await.unwrap();

	let songs: Vec<Songlist> = 
		songs.into_iter().map(|song| Songlist{
			sha1sum: song.sha1sum,
			
			// We are serializing and then immediately deserializing because I can't figure out
			// how to convert the type from Json<Vec<(String, String)>> to Vec<(String, String)>
			content: serde_json::from_str(&serde_json::to_string(&song.content).unwrap()).unwrap(),
		}).collect();
	
	println!("{:?}", songs);



	
	let games: Vec<GameSchema> = sqlx::query_as(
		"SELECT * from games")
		.fetch_all(pool.inner())
		.await.unwrap();


	serde_json::to_string(&games).unwrap()
}
