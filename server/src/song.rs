use std::collections::HashMap;

use rocket::State;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool};

use crate::history::{Songlist, SonglistSchema};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Song {
    pub album: String,
    pub name: String,
    pub lyrics_raw: String,
    pub lines: Vec<Line>,
}

#[derive(Serialize)]
pub struct ISong {
    pub album: String,
    pub name: String,
    pub lyrics_raw: String,
    pub lines: Vec<ILine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Line {
    pub text: String,
    pub is_exclamatory: bool,
    pub has_multiple_successors: bool,
    pub has_bad_successor: bool,
}
#[derive(Debug, Serialize)]
pub struct ILine {
    pub text: String,
    pub is_exclamatory: bool,
    pub has_multiple_successors: bool,
    pub has_bad_successor: bool,
    pub num_guesses: usize,
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.text == other.text
    }
}

impl Line {
    pub fn new(text: &str) -> Line {
        // The goal here is to calculate whether a line is "exclamatory". A line like "Oh, oh, oh, whoa" is exclamatory, since it contains many exclamatory words.
        // We don't want the guessing game's questions to involve exclamatory words, because they are generally difficult to recall or type properly.

        let mut is_exclamatory = false;

        let exclamatory_words = [
            "mmmm", "mmm", "mm", "oh", "ohh", "ooh", "la", "na", "no", "my", "uh", "huh", "ahh",
            "ah", "ha", "yeah", "whoa", "ayy", "i", "eh", "hey", "ra", "di", "da",
        ];

        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower
            .split(|x: char| !x.is_alphabetic())
            .filter(|x| !x.is_empty())
            .collect();
        let num_words = words.len();
        let exclamatory_words = words
            .clone()
            .into_iter()
            .filter(|x| exclamatory_words.contains(x));
        let num_exclamatory_words = exclamatory_words.count();

        if num_exclamatory_words as f32 / num_words as f32 >= 0.5 {
            is_exclamatory = true;
        }
        // we label short lines as exclamatory to exclude them too
        if num_words <= 2 {
            is_exclamatory = true;
        }

        // Nevermind, there was too many false positives with this method. :(
        // // A line is marked as exclamatory if its most common 2 words make up >= 50% of the line's words.
        // let mut word_frequencies: HashMap<&str, i32> = HashMap::new();
        // for word in words {
        // 	if word_frequencies.contains_key(word) {
        // 		word_frequencies.insert(word, word_frequencies.get(word).unwrap() + 1);
        // 	} else {
        // 		word_frequencies.insert(word, 1);
        // 	}
        // }
        // let mut word_freqs: Vec<(&str, i32)> = word_frequencies.into_iter().collect();
        // word_freqs.sort_by(|(_, a2), (_, b2)| a2.partial_cmp(b2).unwrap_or(Ordering::Equal));
        // word_freqs.reverse();
        // let mut num_words_in_two_most_common_words = 0;
        // num_words_in_two_most_common_words += word_freqs.get(0).unwrap_or(&("", 0)).1;
        // num_words_in_two_most_common_words += word_freqs.get(1).unwrap_or(&("", 0)).1;

        // if num_words_in_two_most_common_words as f32 / num_words as f32 >= 0.5 {
        // 	is_exclamatory = true;
        // }

        Line {
            text: text.to_owned(),
            is_exclamatory,
            has_multiple_successors: false,
            has_bad_successor: false,
        }
    }
}

impl Song {
    pub fn new(album: String, name: String, lyrics_raw: String) -> Self {
        let mut lines: Vec<Line> = lyrics_raw
            .split('\n')
            .filter(|x| !(x.starts_with('[') || x.is_empty()))
            .map(|x| x.trim())
            .map(Line::new)
            .collect();

        for index in 0..lines.len() - 1 {
            if lines[index + 1].is_exclamatory {
                lines[index].has_bad_successor = true;
            }
        }
        let n = lines.len();
        lines[n - 1].has_bad_successor = true;

        let mut continuation_map: HashMap<String, Vec<String>> = HashMap::new();
        for index in 0..lines.len() - 1 {
            let line = &lines[index];
            if let Some(continuations) = continuation_map.get(&line.text) {
                if !continuations.contains(&lines[index + 1].text) {
                    let mut v = continuations.clone();
                    v.push(lines[index + 1].text.clone());
                    continuation_map.insert(line.text.clone(), v);
                }
            } else {
                continuation_map.insert(line.text.clone(), vec![lines[index + 1].text.clone()]);
            }
        }
        for index in 0..lines.len() - 1 {
            if let Some(continuations) = continuation_map.get(&lines[index].text) {
                if continuations.len() > 1 {
                    lines[index].has_multiple_successors = true;
                }
            }
        }

        Song {
            album,
            name,
            lyrics_raw,
            lines,
        }
    }
}

// fn line_has_multiple_successors(guess: &Line, lines: &Vec<Line>) -> bool {

// 	let mut possible_continuations = vec![];
// 	for (index, line) in (&lines[..lines.len()-1]).into_iter().enumerate() {
// 		if are_close_enough(line.text.as_str(), guess.text.as_str()) {
// 			possible_continuations.push(lines[index+1].clone());
// 		}
// 	}
// 	for continuation in &possible_continuations {
// 		if continuation.is_exclamatory {
// 			return true;
// 		}
// 	}

// 	for c1 in &possible_continuations {
// 		for c2 in &possible_continuations {
// 			if c1 != c2 {
// 				return true;
// 			}
// 		}
// 	}

// 	false
// }

#[get("/songs")]
pub fn get_song_list(songs: &State<Vec<Song>>) -> String {
    let mut s: HashMap<String, Vec<String>> = HashMap::new();
    for song in songs.iter() {
        if let Some(v) = s.get(&song.album) {
            let mut v = v.clone();
            v.push(song.name.clone());
            s.insert(song.album.clone(), v);
        } else {
            s.insert(song.album.clone(), vec![song.name.clone()]);
        }
    }

    serde_json::to_string(&s).unwrap()
}

#[get("/songs?<id>")]
pub async fn get_song_list_with_id(id: i32, pool: &rocket::State<Pool<MySql>>) -> String {
    let result: Vec<SonglistSchema> = sqlx::query_as("SELECT * FROM songlists WHERE id LIKE ?")
        .bind(id)
        .fetch_all(pool.inner())
        .await
        .unwrap();

    if result.is_empty() {
        return "{}".to_string();
    }

    let songlists: Vec<Songlist> = result
        .into_iter()
        .map(|songlist| Songlist {
            id: songlist.id,
            sha1sum: songlist.sha1sum,

            // We are serializing and then immediately deserializing because I can't figure out
            // how to convert the type from Json<Vec<(String, String)>> to Vec<(String, String)>
            content: serde_json::from_str(&serde_json::to_string(&songlist.content).unwrap())
                .unwrap(),
        })
        .collect();

    let songs = &songlists.get(0).unwrap().content;

    let mut s: HashMap<String, Vec<String>> = HashMap::new();
    for (album, name) in songs.iter() {
        if let Some(v) = s.get(album) {
            let mut v = v.clone();
            v.push(name.clone());
            s.insert(album.clone(), v);
        } else {
            s.insert(album.clone(), vec![name.clone()]);
        }
    }

    serde_json::to_string(&s).unwrap()
}

#[get("/all_songlists")]
pub async fn get_all_songlists(pool: &rocket::State<Pool<MySql>>) -> String {
    let all_songlists: Vec<SonglistSchema> = sqlx::query_as("SELECT * FROM songlists")
        .fetch_all(pool.inner())
        .await
        .unwrap();
    let all_songlists: Vec<Songlist> = all_songlists
        .into_iter()
        .map(|songlist| Songlist {
            id: songlist.id,
            sha1sum: songlist.sha1sum,

            // We are serializing and then immediately deserializing because I can't figure out
            // how to convert the type from Json<Vec<(String, String)>> to Vec<(String, String)>
            content: serde_json::from_str(&serde_json::to_string(&songlist.content).unwrap())
                .unwrap(),
        })
        .collect();

    let mut result: HashMap<i32, &Vec<(String, String)>> = HashMap::new();

    for songlist in all_songlists.iter() {
        result.insert(songlist.id, &songlist.content);
    }

    serde_json::to_string(&result).unwrap()
}

#[derive(FromRow, Debug)]
struct Count {
    total: i32,
}

#[get("/songs/<album>/<name>")]
pub async fn get_song(
    pool: &rocket::State<Pool<MySql>>,
    songs: &State<Vec<Song>>,
    album: &str,
    name: &str,
) -> String {
    for song in songs.iter() {
        if song.album == album && song.name == name {
            let mut my_song = ISong {
                album: song.album.clone(),
                name: song.name.clone(),
                lyrics_raw: song.lyrics_raw.clone(),
                lines: vec![],
            };
            for line in &song.lines {
                let is_exclamatory = line.is_exclamatory;
                let has_multiple_successors = line.has_multiple_successors;
                let has_bad_successor = line.has_bad_successor;
                let mut num_guesses = 0;

                if !is_exclamatory & !has_bad_successor && !has_multiple_successors {
                    let count: Count = sqlx::query_as(
                        "SELECT count(1) as total from guesses 
                        WHERE 
                        album LIKE ?
                        AND song_name LIKE ?
                        AND prompt LIKE ?
                        ",
                    )
                    .bind(album)
                    .bind(name)
                    .bind(line.text.clone())
                    .fetch_one(pool.inner())
                    .await
                    .unwrap();
                    num_guesses = count.total as usize;
                }

                my_song.lines.push(ILine {
                    text: line.text.clone(),
                    is_exclamatory,
                    has_bad_successor,
                    has_multiple_successors,
                    num_guesses,
                })
            }

            return serde_json::to_string(&my_song).unwrap();
        }
    }

    "{}".to_string()
}
