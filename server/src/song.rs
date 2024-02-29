use std::collections::HashMap;

use rocket::State;
use serde::Serialize;
use sqlx::{FromRow, MySql, Pool};

use crate::history::{Songlist, SonglistSchema};

/// Represents a song with an album and songname.
/// lyrics_raw is a string of all lines (separated by `\n`),
/// and lines is a vector of [`Line`] structs containing data about whether lines are good prompts.
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
    /// If the Option is `Some`, that means that the line is a bad prompt,
    /// and the string is a reason why the line is not appropriate to use as a prompt.
    pub is_bad_prompt: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct ILine {
    pub text: String,
    pub is_bad_prompt: Option<String>,
    /// `num_guesses` is the number of times the line has been played in a game. Used by the client to display a subscript.
    pub num_guesses: usize,
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.text == other.text
    }
}

fn calculate_is_exclamatory_heuristic(text: &str) -> bool {
    // The goal here is to calculate whether a line is "exclamatory". A line like "Oh, oh, oh, whoa" is exclamatory, since it contains many exclamatory words.
    // We don't want the guessing game's questions to involve exclamatory words, because they are generally difficult to recall or type properly.
    let exclamatory_words = [
        "mmmm", "mmm", "mm", "oh", "ohh", "ooh", "la", "na", "no", "my", "uh", "huh", "ahh", "ah",
        "ha", "yeah", "whoa", "ayy", "i", "eh", "hey", "ra", "di", "da",
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
        return true;
    }
    // we label short lines as exclamatory too
    if num_words <= 2 {
        return true;
    }
    return false;
}

impl Line {
    pub fn new(raw_text: &str) -> Line {
        let mut is_exclamatory = false;
        let mut is_bad_prompt = None;

        let text = if raw_text.contains('$') {
            // everything before the last `$` is part of the line
            let num_segments = raw_text.split('$').count();
            raw_text
                .split('$')
                .take(num_segments - 1)
                .collect::<Vec<&str>>()
                .join("$")
        } else {
            raw_text.to_string()
        };

        if raw_text.contains('$') {
            let markers = raw_text.split('$').rev().next().unwrap();
            if markers.contains("<exclamatory>") {
                is_exclamatory = true;
            }
            if markers.contains("<misc_bad") {
                let bad_desc = markers
                    .split_once("<misc_bad ")
                    .unwrap()
                    .1
                    .split_once(">")
                    .unwrap()
                    .0;

                is_bad_prompt = Some(bad_desc.to_string());
            }
        }

        if is_exclamatory != calculate_is_exclamatory_heuristic(&text) {
            println!("Warning, the following line's exclamatory status does not match the calculated heuristic:");
            println!("{}", &text);
        }

        Line {
            text: text.to_owned(),
            is_exclamatory,
            is_bad_prompt,
        }
    }
}

impl Song {
    pub fn new(album: String, name: String, lyrics_raw: String) -> Self {
        let mut lyrics_raw_processed = String::new();
        let mut lines: Vec<Line> = Vec::new();
        for raw_line in lyrics_raw.split('\n') {
            let raw_line = raw_line.trim();
            if raw_line.is_empty() {
                continue;
            }
            if raw_line.starts_with('[') {
                lyrics_raw_processed.push_str(raw_line);
                lyrics_raw_processed.push('\n');
                continue;
            }
            let line = Line::new(raw_line);
            lyrics_raw_processed.push_str(&line.text);
            lyrics_raw_processed.push('\n');

            lines.push(line);
        }

        for index in 0..lines.len() - 1 {
            if lines[index + 1].is_exclamatory {
                lines[index].is_bad_prompt = Some("followed by exclamatory line".to_string());
            }
            if lines[index].is_exclamatory {
                lines[index].is_bad_prompt = Some("is an exclamatory line".to_string());
            }
        }

        let n = lines.len();
        lines[n - 1].is_bad_prompt = Some("Has no next line".to_string());

        // NOTE: the logic here calculates whether there are multiple successors
        // let mut continuation_map: HashMap<String, Vec<String>> = HashMap::new();
        // for index in 0..lines.len() - 1 {
        //     let line = &lines[index];
        //     if let Some(continuations) = continuation_map.get(&line.text) {
        //         if !continuations.contains(&lines[index + 1].text) {
        //             let mut v = continuations.clone();
        //             v.push(lines[index + 1].text.clone());
        //             continuation_map.insert(line.text.clone(), v);
        //         }
        //     } else {
        //         continuation_map.insert(line.text.clone(), vec![lines[index + 1].text.clone()]);
        //     }
        // }
        // for index in 0..lines.len() - 1 {
        //     if let Some(continuations) = continuation_map.get(&lines[index].text) {
        //         if continuations.len() > 1 {
        //             lines[index].has_multiple_successors = true;
        //         }
        //     }
        // }

        Song {
            album,
            name,
            lyrics_raw: lyrics_raw_processed,
            lines,
        }
    }
}

/// API endpoint for getting a list of all songs.
/// returns a hashmap, where keys are album names and values are song names.
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

/// API endpoint for getting a list of all songs, from a particular songlist.
/// A songlist is a collection of all available songs, at a particular time.
/// For example, there's a songlist that contains every album released before Midnights.
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

/// API endpoint to get a list of all songlists.
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

/// API endpoint to get a song from album + name.
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
                let is_bad_prompt = &line.is_bad_prompt;
                let mut num_guesses = 0;

                if is_bad_prompt.is_none() {
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
                    is_bad_prompt: is_bad_prompt.clone(),
                    num_guesses,
                })
            }

            return serde_json::to_string(&my_song).unwrap();
        }
    }

    "{}".to_string()
}
