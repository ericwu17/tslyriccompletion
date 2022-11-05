use rand::Rng;
use rocket::State;
use rocket::serde::json::Json;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use crate::song::Song;
use crate::guess_generating::{pick_random_guess, optimal_truncated_dist, pick_distractors, Question};
use crate::diff::diff_greedy;
use crate::lifelines::{LifelineInventory, Lifeline};
use rand::prelude::SliceRandom;
use std::collections::{HashMap};
use uuid::Uuid;
use sqlx::{Pool, MySql};
use sha1::{Sha1, Digest};



const MAX_ACCEPTABLE_DIST: usize = 13;
const POINTS_FOR_PERFECT_MATCH: i32 = 26;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum Hint {
	ShowTitle(String),
	ShowPrevLines{lines: String, is_at_song_beginning: bool},
	Skip,
}

impl Hint {
	pub fn underlying_lifeline(&self) -> Lifeline{
		match self {
			Hint::ShowTitle(_) => Lifeline::ShowTitleAlbum,
			Hint::ShowPrevLines{..} => Lifeline::ShowPrevLines,
			Hint::Skip => Lifeline::Skip,
		}
	}
}

#[derive(Clone, Debug)]
pub struct GameState {
	score: i32,
	guesses_made: i32,
	current_question: Question,
	lifeline_inv: LifelineInventory,
	hints_shown: Vec<Hint>,
	choices: Vec<String>,
	terminated: bool,
	completed_question: bool,
	included_songs: Vec<(String, String)>,
}

#[derive(Serialize)]
pub struct GameStatePublic {
	id: String,
	score: i32,
	guesses_made: i32,
	current_question: Question,
	lifeline_inv: LifelineInventory,
	hints_shown: Vec<Hint>,
	choices: Vec<String>,
	terminated: bool,
	included_songs: Vec<(String, String)>,
	completed_question: bool,
}

#[derive(Serialize)]
pub struct FlaggedString {
	flags: Vec<i32>,
	text: String,
}

impl FlaggedString {
	pub fn set_all_flags(&mut self, val: i32) {
		for i in 0..self.flags.len() {
			self.flags[i] = val;
		}
	}
}

#[derive(Serialize)]
pub enum GuessResult {
	AFM { // asking for more. Used when the user's guess is on the right track but too short.
		target_length: usize,
		guess_length: usize,
	},
	Correct {
		points_earned: i32,
		user_guess: FlaggedString,
		answer: FlaggedString,
		new_lifeline: Option<Lifeline>,
	},
	Incorrect {
		user_guess: FlaggedString,
		answer: FlaggedString,
	},
}

#[derive(Serialize)]
pub struct GuessResultPublic {
	guess_res: GuessResult,
	game_state: GameStatePublic,
}

impl GameState {
	pub fn new(songs: &Vec<Song>,songs_to_include: &mut Vec<(String, String)>) -> Self {
		// This function will modify songs_to_include, so that if it's the empty vector,
		// it will end up containing all songs in songs. It will also filter out any
		// invalid songs.
		let mut actual_songs_to_include: Vec<(String, String)> = songs_to_include.clone().into_iter()
			.filter(|(a, b)| {
				songs.iter().filter(|song| song.album == *a && song.name == *b).count() > 0
			}
			).collect();
		if actual_songs_to_include.len() == 0 {
			actual_songs_to_include = songs.iter().map(|song| (song.album.clone(), song.name.clone())).collect();
		}
		*songs_to_include = actual_songs_to_include.clone();
		GameState {
			score: 0, 
			guesses_made: 0, 
			current_question: pick_random_guess(songs, &actual_songs_to_include), 
			lifeline_inv: LifelineInventory::new(), 
			hints_shown: vec![], 
			choices: vec![],
			terminated: false,
			completed_question: false,
			included_songs: actual_songs_to_include,
		}
	}


	pub fn into_public(&self, id: String) -> GameStatePublic {
		GameStatePublic {
			score: self.score,
			guesses_made: self.guesses_made,
			current_question: Question {
				song: Song{
					album: String::new(), name: String::new(), lyrics_raw: String::new(), lines: vec![]
				},
				shown_line: self.current_question.shown_line.clone(),
				answer: String::new(),
			},
			lifeline_inv: self.lifeline_inv.clone(),
			hints_shown: self.hints_shown.clone(),
			choices: self.choices.clone(),
			id,
			terminated: self.terminated,
			included_songs: self.included_songs.clone(),
			completed_question: self.completed_question,
		}
	}
	pub fn into_public_with_answers(&self, id: String) -> GameStatePublic {
		GameStatePublic {
			score: self.score,
			guesses_made: self.guesses_made,
			current_question: self.current_question.clone(),
			lifeline_inv: self.lifeline_inv.clone(),
			hints_shown: self.hints_shown.clone(),
			choices: self.choices.clone(),
			id,
			terminated: self.terminated,
			included_songs: self.included_songs.clone(),
			completed_question: self.completed_question,
		}
	}

	pub fn has_used_lifeline(&self, lifeline: Lifeline) -> bool {
		match lifeline {
			Lifeline::ShowPrevLines => {
				for hint in &self.hints_shown {
					if let Hint::ShowPrevLines{..} = hint {
						return true;
					}
				}
				return false;
			},
			Lifeline::ShowTitleAlbum => {
				for hint in &self.hints_shown {
					if let Hint::ShowTitle(_) = hint {
						return true;
					}
				}
				return false;
			},
			Lifeline::Skip => {
				for hint in &self.hints_shown {
					if *hint == Hint::Skip {
						return true;
					}
				}
				return false;
			}
		}
	}

	pub fn increment_score(&mut self) {
		self.score += 1;
	}
}


#[post("/game/start", format = "application/json", data = "<songs_to_include>")]
pub async fn init_game(game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>, songs: &State<Vec<Song>>, songs_to_include: Json<Vec<(String, String)>>, pool: &rocket::State<Pool<MySql>>) -> String {
	let mut songs_to_include = songs_to_include.to_vec();
	let new_game_state;
	let uuid = Uuid::new_v4().to_string();
	new_game_state = GameState::new(songs, &mut songs_to_include);

	{
		let mut guard = game_state.lock().unwrap();
		(*guard).insert(uuid.clone(), new_game_state.clone());
	}

	let full_songlist: Vec<(String, String)> = songs.iter().map(|song| (song.album.clone(), song.name.clone())).collect();
	let mut songlist_desc: HashMap<String, Vec<bool>> = HashMap::new();
	// The for loop below builds out the songlist_desc object, which is a Hashmap mapping album names to a list of boolean values.
	// The list of boolean values represents which songs are included/excluded in the game.
	for song in &full_songlist {
		let is_included = songs_to_include.contains(&song);
		if let Some(v) = songlist_desc.get(&song.0) {
			let mut v = v.clone();
			v.push(is_included);
			songlist_desc.insert(song.0.clone(), v);
		} else {
			songlist_desc.insert(song.0.clone(), vec![is_included]);
		}
	}

	let mut hasher = Sha1::new();
	hasher.update(serde_json::to_string(&full_songlist).unwrap().as_bytes());
	let full_songlist_hash = format!("{:X}", hasher.finalize());


	let full_songlist_json: sqlx::types::Json<Vec<(String, String)>> = sqlx::types::Json(songs_to_include);
	let songlist_desc_json = sqlx::types::Json(songlist_desc);
	
	let _ = sqlx::query("INSERT INTO songlists VALUES (?, ?)")
		.bind(full_songlist_hash.clone())
		.bind(full_songlist_json)
		.fetch_all(pool.inner())
		.await;
	let _ = sqlx::query("INSERT INTO games VALUES (?, NOW(), ?, ?, 0, NULL, NULL)")
		.bind(uuid.clone())
		.bind(full_songlist_hash.clone())
		.bind(songlist_desc_json)
		.fetch_all(pool.inner())
		.await;

	serde_json::to_string(&new_game_state.into_public(uuid.clone())).unwrap()
}

#[get("/game/use-lifeline?<id>&<lifeline>")]
pub async fn game_lifelines(game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>, id: String, lifeline: &str, pool: &rocket::State<Pool<MySql>>) -> String {
	let res = 'outer_block: {
		let mut guard = game_state.lock().unwrap();
		if let Some(game_state) = (*guard).get(&id) {
			let mut new_game_state = game_state.clone();
			match lifeline {
				"show_title_album" => {
					if !new_game_state.has_used_lifeline(Lifeline::ShowTitleAlbum)
							&& new_game_state.lifeline_inv.consume_lifeline(Lifeline::ShowTitleAlbum) {
						let title = format!("{} : {}", game_state.current_question.song.album, game_state.current_question.song.name);
						new_game_state.hints_shown.push(Hint::ShowTitle(title));
						(*guard).insert(id.clone(), new_game_state.clone());
						return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap()
					} else {
						// no lifelines remaining, so do nothing
						return serde_json::to_string(&game_state.into_public(id.clone())).unwrap()
					}
				},
				"show_prev_lines" => {
					if !new_game_state.has_used_lifeline(Lifeline::ShowPrevLines)
							&& new_game_state.lifeline_inv.consume_lifeline(Lifeline::ShowPrevLines) {
						let (lines, is_at_song_beginning) = get_previous_lines(&new_game_state.current_question);
						new_game_state.hints_shown.push(Hint::ShowPrevLines{lines, is_at_song_beginning});
						(*guard).insert(id.clone(), new_game_state.clone());
						return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap()
					} else {
						// no lifelines remaining, so do nothing
						return serde_json::to_string(&game_state.into_public(id)).unwrap()
					}
				},
				"skip" => {
					if !new_game_state.has_used_lifeline(Lifeline::Skip)
							&& new_game_state.lifeline_inv.consume_lifeline(Lifeline::Skip) {
						new_game_state.hints_shown.push(Hint::Skip);
						new_game_state.completed_question = true;
						(*guard).insert(id.clone(), new_game_state.clone());
						// not calling into_public() because we want to show everything, including all answers.
						break 'outer_block new_game_state;
					} else {
						// no lifelines remaining, so do nothing
						return serde_json::to_string(&game_state.into_public(id.clone())).unwrap()
					}
				},
				_ => {},
			}
		}

		return "{}".to_owned()
	};
	let gs = res.clone();

	let _ = sqlx::query("INSERT INTO guesses VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())")
		.bind(id.clone())
		.bind(gs.guesses_made)
		.bind(gs.current_question.song.album)
		.bind(gs.current_question.song.name)
		.bind(gs.current_question.shown_line)
		.bind(gs.current_question.answer)
		.bind("skipped")
		.bind("")
		.bind(0)
		.bind(Option::<String>::None)
		.bind(
			sqlx::types::Json(gs.hints_shown.iter().map(|hint| hint.underlying_lifeline().as_string()).collect::<Vec<String>>())
		)
		.bind(sqlx::types::Json(gs.choices))
		.fetch_all(pool.inner())
		.await;

	return serde_json::to_string(&res.into_public_with_answers(id.clone())).unwrap() 
}


#[get("/game/reduce-multiple-choice?<id>")]
pub fn reduce_multiple_choice(game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>, songs: &State<Vec<Song>>, id: String) -> String {
	let mut guard = game_state.lock().unwrap();
	if let Some(game_state) = (*guard).get(&id) {
		if game_state.choices.len() > 0 {
			// we do nothing if the current game state has already been reduced to multiple choice
			return serde_json::to_string(&game_state.into_public(id.clone())).unwrap()
		}

		let mut new_game_state = game_state.clone();
		let answer = new_game_state.current_question.answer.clone();
		new_game_state.choices = pick_distractors(&answer, songs);
		new_game_state.choices.push(answer);
		new_game_state.choices.shuffle(&mut rand::thread_rng());

		(*guard).insert(id.clone(), new_game_state.clone());
		return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap()

	}

	"{}".to_owned()
}

#[get("/game/next?<id>")]
pub fn next_question(game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>, songs: &State<Vec<Song>>, id: String) -> String {
	let mut guard = game_state.lock().unwrap();
	if let Some(game_state) = (*guard).get(&id) {
		if game_state.completed_question && !game_state.terminated {
			let mut new_game_state = game_state.clone();
			new_game_state.current_question = pick_random_guess(songs, &game_state.included_songs);
			new_game_state.completed_question = false;
			new_game_state.choices = vec![];
			new_game_state.hints_shown = vec![];
			new_game_state.guesses_made += 1;

			(*guard).insert(id.clone(), new_game_state.clone());
			return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap()

		} else {
			return serde_json::to_string(&game_state.into_public(id.clone())).unwrap()
		}
	}

	"{}".to_owned()
}

#[get("/game/submit-guess?<id>&<guess>")]
pub async fn take_guess(game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>, id: String, guess: &str, pool: &rocket::State<Pool<MySql>>) -> String {
	let outer_game_state:GameState;
	let guess_res = 'outer_block: {
		let mut guard = game_state.lock().unwrap();
		if let Some(game_state) = (*guard).get(&id) {
			if game_state.completed_question {
				// already guessed, so we do nothing
				return serde_json::to_string(&game_state.into_public_with_answers(id)).unwrap();
			}


			let question = game_state.current_question.clone();

			if guess.chars().count() < question.answer.chars().count() - 5 && guess != "/" && is_on_right_track(&guess, &question.answer) && game_state.choices.len() == 0 
				|| guess.chars().count() > 150 {
				// The guess was on the right track, but was too short.
				// We also return AFM (refuse to process the guess) if the user submits a ridiculously long guess.
				let res = GuessResultPublic {
					game_state: game_state.into_public(id),
					guess_res: GuessResult::AFM{ 
						target_length:question.answer.chars().count(),
						guess_length: guess.chars().count(),
					},
				};
				return serde_json::to_string(&res).unwrap();
			}

			let (truncate_amt, dist) = optimal_truncated_dist(&guess, &question.answer);
			let mut new_game_state = game_state.clone();
			let (mut guess_flag_str, mut ans_flag_str) = get_flags(guess, &question.answer, truncate_amt);
			let points_earned: i32;
			let mut maybe_new_lifeline = None;

			if (game_state.choices.len() == 0 && dist > MAX_ACCEPTABLE_DIST) || (game_state.choices.len() > 0 && guess != question.answer) {
				// The user has guessed wrong and the game is now over
				if game_state.choices.len() > 0 {
					// In a multiple choice situation, we set the flags for both strings' characters all to red.
					guess_flag_str.set_all_flags(1);
					ans_flag_str.set_all_flags(1);
				}
				new_game_state.terminated = true;
				new_game_state.completed_question = true;
				(*guard).insert(id.clone(), new_game_state.clone());
				
				let res = GuessResultPublic {
					game_state: new_game_state.into_public_with_answers(id.clone()),
					guess_res: GuessResult::Incorrect {
						user_guess: guess_flag_str,
						answer: ans_flag_str,
					},
				};
				outer_game_state = new_game_state.clone();
				break 'outer_block res;
			} else if game_state.choices.len() > 0 {
				// The user got a multiple choice question correct
				points_earned = 1;
			} else if dist != 0 {
				// The guess was correct but not perfect.
				if rand::thread_rng().gen_range(0..MAX_ACCEPTABLE_DIST) > dist {
					maybe_new_lifeline = Some(Lifeline::random_lifeline());
				}
				points_earned = (MAX_ACCEPTABLE_DIST - dist + 1) as i32;
			} else {
				// perfect match
				maybe_new_lifeline = Some(Lifeline::random_lifeline());
				points_earned = POINTS_FOR_PERFECT_MATCH;
			}

			
			new_game_state.score += points_earned;
			new_game_state.completed_question = true;
			if let Some(new_lifeline) = &maybe_new_lifeline {
				new_game_state.lifeline_inv.add_lifeline(&new_lifeline);
			}
			(*guard).insert(id.clone(), new_game_state.clone());

			let res = GuessResultPublic {
				game_state: new_game_state.into_public_with_answers(id.clone()),
				guess_res: GuessResult::Correct {
					points_earned,
					user_guess: guess_flag_str,
					answer: ans_flag_str,
					new_lifeline: maybe_new_lifeline
				}
			};
			outer_game_state = new_game_state.clone();
			break 'outer_block res;
		}
		return "{}".to_owned()
	};

	let gs = outer_game_state;

	let num_points_earned;
	let is_correct;
	let mut lifeline_earned: Option<String> = None;
	match &guess_res.guess_res {
		GuessResult::Correct { points_earned, new_lifeline, .. } => {
			num_points_earned = *points_earned;
			is_correct = true;
			lifeline_earned = match new_lifeline {
				None => None,
				Some(lifeline) => Some(lifeline.as_string()),
			};
		},
		GuessResult::Incorrect { .. } => {
			num_points_earned = 0;
			is_correct = false;
		},
		_ => {unreachable!()}
	}
	// If the code runs to this point, then the guess is either correct or incorrect (not AFM state).
	// We can now record the guess into the database before returning.

	let _ = sqlx::query("INSERT INTO guesses VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())")
		.bind(id.clone())
		.bind(gs.guesses_made)
		.bind(gs.current_question.song.album)
		.bind(gs.current_question.song.name)
		.bind(gs.current_question.shown_line)
		.bind(gs.current_question.answer)
		.bind(if is_correct {"correct"} else {"incorrect"})
		.bind(guess)
		.bind(num_points_earned)
		.bind(lifeline_earned)
		.bind(
			sqlx::types::Json(gs.hints_shown.iter().map(|hint| hint.underlying_lifeline().as_string()).collect::<Vec<String>>())
		)
		.bind(sqlx::types::Json(gs.choices))
		.fetch_all(pool.inner())
		.await;
	
	if !is_correct {
		let _ = sqlx::query(
		"UPDATE games 
			SET 
				has_terminated = true,
				terminal_score = ?
			WHERE
				UUID = ?
		")
		.bind(gs.score)
		.bind(id.clone())
		.fetch_all(pool.inner())
		.await;
	}

	return serde_json::to_string(&guess_res).unwrap();
}


fn get_flags(guess: &str, answer: &str, optimal_truncate_amt: i32) -> (FlaggedString, FlaggedString){
	let (_, diffs) = 
		diff_greedy(
			&guess.to_lowercase()[0..(guess.len()-optimal_truncate_amt as usize)], 
			&answer.to_lowercase(),
		).unwrap();
	
	let mut guess_flags = vec![0; guess.chars().count()];
	let mut ans_flags = vec![0; answer.chars().count()];
	for insertion in diffs.get("insert").unwrap() {
		for i in insertion.at..=insertion.to {
			ans_flags[i] = 1;
		}
	}
	for deletion in diffs.get("delete").unwrap() {
		for i in deletion.at..=deletion.to {
			guess_flags[i] = 1;
		}
	}
	let num_chars_truncated = guess[(guess.len()-optimal_truncate_amt as usize)..].chars().count();

	for i in (guess_flags.len()-num_chars_truncated)..guess_flags.len() {
		guess_flags[i] = 2;
	}

	(
		FlaggedString {
			flags: guess_flags,
			text: guess.to_string(),
		},
		FlaggedString {
			flags: ans_flags,
			text: answer.to_string(),
		}
	)
}


pub fn is_on_right_track(guess: &str, answer: &str) -> bool {
	let (_, dist) = optimal_truncated_dist(answer, guess);
	dist <= MAX_ACCEPTABLE_DIST
}

fn get_previous_lines(question: &Question) -> (String, bool) {
	const PREV_LINES_TO_SHOW: usize = 2;

	let lines = question.song.lines.clone();
	let mut answer_position: usize = 0;
	for index in 0..lines.len() {
		if lines[index].text == question.shown_line {
			answer_position = index;
		}
	}
	let mut output = String::new();

	let is_at_song_beginning =  answer_position <= PREV_LINES_TO_SHOW;
	let beginning_index = std::cmp::max(answer_position as i32 - PREV_LINES_TO_SHOW as i32, 0);

	for index in (beginning_index as usize)..=answer_position {
		output.push_str(&format!("{}\n", lines[index].text));
	}
	(output, is_at_song_beginning)

}
