use crate::guess_generating::{
    lowercase_ignore_punctuation_edit_dist, optimal_truncated_dist, pick_distractors,
    pick_random_guess, Question,
};
use crate::history::{Songlist, SonglistSchema};
use crate::lifelines::{Lifeline, LifelineInventory};
use crate::song::Song;
use rand::prelude::SliceRandom;
use rand::Rng;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sha1::{Digest, Sha1};
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// These characters are to be ignored when taking the edit distance between two strings:
pub const CHARS_TO_IGNORE: &[char] = &['(', ')', ',', '.', '-', ':', ';', '"', '\'', '?', ' '];

/// If a guess's dist is greater than `MAX_ACCEPTABLE_DIST` from the answer, then the game ends.
const MAX_ACCEPTABLE_DIST: usize = 13;
/// A bonus is awarded the guess matches the answer perfectly.
const POINTS_FOR_PERFECT_MATCH: i32 = 26;

/// An enum representing a shown hint.
/// The `Skip` variant is classified as a hint, even though it isn't really a hint,
/// more of a lifeline.
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub enum Hint {
    ShowTitle(String),
    ShowPrevLines {
        lines: String,
        is_at_song_beginning: bool,
    },
    Skip,
}

impl Hint {
    pub fn underlying_lifeline(&self) -> Lifeline {
        match self {
            Hint::ShowTitle(_) => Lifeline::ShowTitleAlbum,
            Hint::ShowPrevLines { .. } => Lifeline::ShowPrevLines,
            Hint::Skip => Lifeline::Skip,
        }
    }
}

/// A struct representing the current state of a single game in progress.
#[derive(Clone, Debug)]
pub struct GameState {
    /// The player's current score.
    score: i32,
    /// The number of total guesses made so far.
    guesses_made: i32,
    /// The current question shown to the player.
    current_question: Question,
    /// The player's currently available lifelines.
    lifeline_inv: LifelineInventory,
    /// A collection of the hints shown to the player for the current question.
    /// This vector gets reset to an empty vec when the player moves on to the next question.
    hints_shown: Vec<Hint>,
    /// A vector of answer choices, or empty if the current question is not in multiple-choice mode.
    choices: Vec<&'static str>,
    /// True if the game has ended.
    terminated: bool,
    /// True if the question has been completed but the next question has not been requested.
    completed_question: bool,
    /// A vector of songs included in the game. The (str, str) pairs are
    /// (Album_name, Song_name) pairs.
    included_songs: Vec<(&'static str, &'static str)>,
}

/// A struct related to [`GameState`]
/// Used in responses to the player, while [`GameState`] is used by the server to store internal state.
#[derive(Serialize)]
pub struct GameStatePublic {
    id: String,
    score: i32,
    guesses_made: i32,
    current_question: Question,
    lifeline_inv: LifelineInventory,
    hints_shown: Vec<Hint>,
    choices: Vec<&'static str>,
    terminated: bool,
    included_songs: Vec<(&'static str, &'static str)>,
    completed_question: bool,
}

/// A struct representing a result of a player's guess.
#[derive(Serialize)]
pub enum GuessResult {
    /// Asking for more. Used when the player's guess is on the right track but too short.
    /// In this case, we tell the player the target_length.
    AFM {
        target_length: usize,
        guess_length: usize,
    },
    /// A correct response. We tell the player how many points they earned, as well as
    /// a diffed comparison of their answer vs the correct answer.
    /// We also tell them which new lifeline they earned, if any.
    Correct {
        points_earned: i32,
        user_guess: String,
        answer: String,
        new_lifeline: Option<Lifeline>,
    },
    /// An incorrect answer.
    Incorrect { user_guess: String, answer: String },
}

/// This is a combination of a [`GuessResult`] and a [`GameState`] sent back to the player
/// after each guess.
#[derive(Serialize)]
pub struct GuessResultPublic {
    guess_res: GuessResult,
    game_state: GameStatePublic,
}

impl GameState {
    /// Create a new GameState, representing a game played with a subset of songs in `songs_to_include`
    ///
    /// This function will modify the argument `songs_to_include`, so that if it's the empty vector,
    /// it will end up containing all songs in songs. It will also filter out any
    /// invalid songs in `songs_to_include`.
    pub fn new(songs: &[Song], songs_to_include: &mut Vec<(&str, &str)>) -> Self {
        let mut actual_songs_to_include: Vec<(&'static str, &'static str)> = songs_to_include
            .clone()
            .into_iter()
            .filter_map(|(a, b)| {
                songs
                    .iter()
                    .find(|song| song.album == a && song.name == b)
                    .map(|song| (song.album, song.name))
            })
            .collect();
        if actual_songs_to_include.is_empty() {
            actual_songs_to_include = songs.iter().map(|song| (song.album, song.name)).collect();
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

    /// convert a [`GameState`] into a [`GameStatePublic`], with the answer hidden.
    /// since the GameState does not have a UUID, it must be provided.
    pub fn into_public(&self, id: String) -> GameStatePublic {
        GameStatePublic {
            score: self.score,
            guesses_made: self.guesses_made,
            current_question: self.current_question.hide_answer_and_song(),
            lifeline_inv: self.lifeline_inv.clone(),
            hints_shown: self.hints_shown.clone(),
            choices: self.choices.clone(),
            id,
            terminated: self.terminated,
            included_songs: self.included_songs.clone(),
            completed_question: self.completed_question,
        }
    }

    /// convert a [`GameState`] into a [`GameStatePublic`], with the answer shown.
    /// since the GameState does not have a UUID, it must be provided.
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

    /// returns whether a lifeline `lifeline` has been used in the current question
    pub fn has_used_lifeline(&self, lifeline: Lifeline) -> bool {
        match lifeline {
            Lifeline::ShowPrevLines => {
                for hint in &self.hints_shown {
                    if let Hint::ShowPrevLines { .. } = hint {
                        return true;
                    }
                }
                false
            }
            Lifeline::ShowTitleAlbum => {
                for hint in &self.hints_shown {
                    if let Hint::ShowTitle(_) = hint {
                        return true;
                    }
                }
                false
            }
            Lifeline::Skip => {
                for hint in &self.hints_shown {
                    if *hint == Hint::Skip {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn set_single_answer(&mut self, ans: &'static str) {
        self.current_question.answers = vec![ans];
    }
}

/// API endpoint to start a new game.
#[post(
    "/game/start",
    format = "application/json",
    data = "<songs_to_include>"
)]
pub async fn init_game(
    game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>,
    songs: &State<Vec<Song>>,
    songs_to_include: Json<Vec<(&str, &str)>>,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let mut songs_to_include = songs_to_include.to_vec();
    let new_game_state = GameState::new(songs, &mut songs_to_include);
    let uuid = Uuid::new_v4().to_string();

    {
        let mut guard = game_state.lock().unwrap();
        (*guard).insert(uuid.clone(), new_game_state.clone());
    }

    let full_songlist: Vec<(&'static str, &'static str)> =
        songs.iter().map(|song| (song.album, song.name)).collect();
    let mut songlist_desc: HashMap<&'static str, Vec<bool>> = HashMap::new();
    // The for loop below builds out the songlist_desc object, which is a Hashmap mapping album names to a list of boolean values.
    // The list of boolean values represents which songs are included/excluded in the game.
    for song in &full_songlist {
        let is_included = songs_to_include.contains(song);
        if let Some(v) = songlist_desc.get(song.0) {
            let mut v = v.clone();
            v.push(is_included);
            songlist_desc.insert(song.0, v);
        } else {
            songlist_desc.insert(song.0, vec![is_included]);
        }
    }

    let mut hasher = Sha1::new();
    hasher.update(serde_json::to_string(&full_songlist).unwrap().as_bytes());
    let full_songlist_hash = format!("{:X}", hasher.finalize());

    let full_songlist_json: sqlx::types::Json<Vec<(&'static str, &'static str)>> =
        sqlx::types::Json(full_songlist);
    let songlist_desc_json = sqlx::types::Json(songlist_desc);

    // check if the current songlist SHA already exists
    let result: Vec<SonglistSchema> =
        sqlx::query_as("SELECT * FROM songlists WHERE sha1sum LIKE ?")
            .bind(full_songlist_hash.clone())
            .fetch_all(pool.inner())
            .await
            .unwrap();

    println!("result has length {}", result.len());

    if result.is_empty() {
        // insert a new record if the current songlist sha is not found
        let _ = sqlx::query("INSERT INTO songlists (sha1sum, content) VALUES (?, ?)")
            .bind(full_songlist_hash.clone())
            .bind(full_songlist_json)
            .fetch_all(pool.inner())
            .await;
    }

    let songlists: Vec<SonglistSchema> =
        sqlx::query_as("SELECT * FROM songlists WHERE sha1sum LIKE ?")
            .bind(full_songlist_hash.clone())
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

    let songlist_id = songlists
        .first()
        .expect(
            "Expect to have one songlist with appropriate SHA1 sum after inserting the songlist",
        )
        .id;

    // save the game to database
    let _ = sqlx::query("INSERT INTO games VALUES (?, NOW(), ?, ?, 0, NULL, NULL)")
        .bind(uuid.clone())
        .bind(songlist_id)
        .bind(songlist_desc_json)
        .fetch_all(pool.inner())
        .await;

    serde_json::to_string(&new_game_state.into_public(uuid.clone())).unwrap()
}

/// API endpoint to use a lifeline specified by `lifeline`.
#[get("/game/use-lifeline?<id>&<lifeline>")]
pub async fn game_lifelines(
    game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>,
    id: String,
    lifeline: &str,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let res = 'outer_block: {
        let mut guard = game_state.lock().unwrap();
        if let Some(game_state) = (*guard).get(&id) {
            let mut new_game_state = game_state.clone();
            match lifeline {
                "show_title_album" => {
                    if !new_game_state.has_used_lifeline(Lifeline::ShowTitleAlbum)
                        && new_game_state
                            .lifeline_inv
                            .consume_lifeline(Lifeline::ShowTitleAlbum)
                    {
                        let title = format!(
                            "{} : {}",
                            game_state.current_question.song.album,
                            game_state.current_question.song.name
                        );
                        new_game_state.hints_shown.push(Hint::ShowTitle(title));
                        (*guard).insert(id.clone(), new_game_state.clone());
                        return serde_json::to_string(&new_game_state.into_public(id.clone()))
                            .unwrap();
                    } else {
                        // no lifelines remaining, so do nothing
                        return serde_json::to_string(&game_state.into_public(id.clone())).unwrap();
                    }
                }
                "show_prev_lines" => {
                    if !new_game_state.has_used_lifeline(Lifeline::ShowPrevLines)
                        && new_game_state
                            .lifeline_inv
                            .consume_lifeline(Lifeline::ShowPrevLines)
                    {
                        let (lines, is_at_song_beginning) =
                            get_previous_lines(&new_game_state.current_question);
                        new_game_state.hints_shown.push(Hint::ShowPrevLines {
                            lines,
                            is_at_song_beginning,
                        });
                        (*guard).insert(id.clone(), new_game_state.clone());
                        return serde_json::to_string(&new_game_state.into_public(id.clone()))
                            .unwrap();
                    } else {
                        // no lifelines remaining, so do nothing
                        return serde_json::to_string(&game_state.into_public(id)).unwrap();
                    }
                }
                "skip" => {
                    if !new_game_state.has_used_lifeline(Lifeline::Skip)
                        && new_game_state.lifeline_inv.consume_lifeline(Lifeline::Skip)
                    {
                        new_game_state.hints_shown.push(Hint::Skip);
                        new_game_state.completed_question = true;
                        (*guard).insert(id.clone(), new_game_state.clone());
                        // not calling into_public() because we want to show everything, including all answers.
                        break 'outer_block new_game_state;
                    } else {
                        // no lifelines remaining, so do nothing
                        return serde_json::to_string(&game_state.into_public(id.clone())).unwrap();
                    }
                }
                _ => {}
            }
        }

        return "{}".to_owned();
    };
    let gs = res.clone();

    let answer = gs
        .current_question
        .answers
        .choose(&mut rand::thread_rng())
        .unwrap();

    let _ = sqlx::query("INSERT INTO guesses VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())")
        .bind(id.clone())
        .bind(gs.guesses_made)
        .bind(gs.current_question.song.album)
        .bind(gs.current_question.song.name)
        .bind(gs.current_question.shown_line)
        .bind(answer)
        .bind("skipped")
        .bind("")
        .bind(0)
        .bind(Option::<String>::None)
        .bind(sqlx::types::Json(
            gs.hints_shown
                .iter()
                .map(|hint| hint.underlying_lifeline().as_string())
                .collect::<Vec<String>>(),
        ))
        .bind(sqlx::types::Json(gs.choices))
        .fetch_all(pool.inner())
        .await;

    serde_json::to_string(&res.into_public_with_answers(id.clone())).unwrap()
}

/// API endpoint to turn the current question into multiple choice.
/// Returns the new [`GameState`]
#[get("/game/reduce-multiple-choice?<id>")]
pub fn reduce_multiple_choice(
    game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>,
    songs: &State<Vec<Song>>,
    id: String,
) -> String {
    let mut guard = game_state.lock().unwrap();
    if let Some(game_state) = (*guard).get(&id) {
        if !game_state.choices.is_empty() {
            // we do nothing if the current game state has already been reduced to multiple choice
            return serde_json::to_string(&game_state.into_public(id.clone())).unwrap();
        }

        let mut new_game_state = game_state.clone();
        let answers = new_game_state.current_question.answers.clone();

        // we pick the first answer. The answers vec will have already been shuffled.
        // it's important to pick the first one to remain consistent with the showPrevLines behavior
        let answer = *answers.first().unwrap();

        new_game_state.choices = pick_distractors(answers, songs);
        new_game_state.choices.push(answer);
        new_game_state.choices.shuffle(&mut rand::thread_rng());
        // the question should now have only a single answer
        new_game_state.set_single_answer(answer);

        (*guard).insert(id.clone(), new_game_state.clone());
        return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap();
    }

    "{}".to_owned()
}

/// API endpoint to advance to the next question. Does nothing if the current question is not completed.
/// Returns the new [`GameState`]
#[get("/game/next?<id>")]
pub fn next_question(
    game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>,
    songs: &State<Vec<Song>>,
    id: String,
) -> String {
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
            return serde_json::to_string(&new_game_state.into_public(id.clone())).unwrap();
        } else {
            return serde_json::to_string(&game_state.into_public(id.clone())).unwrap();
        }
    }

    "{}".to_owned()
}

/// API endpoint to claim a game
/// When a game first ends after an incorrect response, the game is "unclaimed", and so the player name
/// will be NULL in the database. If the player enters their name, this API endpoint will be called.
#[get("/game/claim?<id>&<name>")]
pub async fn claim_game(id: String, name: String, pool: &rocket::State<Pool<MySql>>) -> String {
    let _ = sqlx::query(
        "UPDATE games
			SET
				player_name = ?
			WHERE
				UUID = ?
		",
    )
    .bind(name)
    .bind(id)
    .fetch_all(pool.inner())
    .await;

    "{}".to_owned()
}

/// Submit a guess for a game.
#[get("/game/submit-guess?<id>&<guess>")]
pub async fn take_guess(
    game_state: &State<Arc<Mutex<HashMap<String, GameState>>>>,
    id: String,
    guess: &str,
    pool: &rocket::State<Pool<MySql>>,
) -> String {
    let outer_game_state: GameState;
    let mut closest_answer;
    let guess_res = 'outer_block: {
        let mut guard = game_state.lock().unwrap();
        if let Some(game_state) = (*guard).get(&id) {
            closest_answer = game_state.current_question.answers[0];
            if game_state.completed_question {
                // already guessed, so we do nothing
                return serde_json::to_string(&game_state.into_public_with_answers(id)).unwrap();
            }
            if guess.chars().count() > 150 {
                // We also return AFM (refuse to process the guess) if the user submits a ridiculously long guess.
                let res = GuessResultPublic {
                    game_state: game_state.into_public(id),
                    guess_res: GuessResult::AFM {
                        target_length: 0,
                        guess_length: guess.chars().count(),
                    },
                };
                return serde_json::to_string(&res).unwrap();
            }

            // HANDLE MULTIPLE CHOICE (inside this if statement)
            if !game_state.choices.is_empty() {
                let correct_answer = game_state.current_question.answers[0];

                let mut new_game_state = game_state.clone();

                if guess == correct_answer {
                    // The user guessed correctly on a multiple choice question
                    new_game_state.score += 1;
                    new_game_state.completed_question = true;
                    (*guard).insert(id.clone(), new_game_state.clone());

                    let res = GuessResultPublic {
                        game_state: new_game_state.into_public_with_answers(id.clone()),
                        guess_res: GuessResult::Correct {
                            points_earned: 1,
                            user_guess: guess.to_owned(),
                            answer: correct_answer.to_owned(),
                            new_lifeline: None,
                        },
                    };
                    outer_game_state = new_game_state.clone();
                    break 'outer_block res;
                } else {
                    // The user has guessed wrong and the game is now over
                    new_game_state.terminated = true;
                    new_game_state.completed_question = true;
                    (*guard).remove(&id);

                    let res = GuessResultPublic {
                        game_state: new_game_state.into_public_with_answers(id.clone()),
                        guess_res: GuessResult::Incorrect {
                            user_guess: guess.to_owned(),
                            answer: correct_answer.to_owned(),
                        },
                    };
                    outer_game_state = new_game_state.clone();
                    break 'outer_block res;
                }
            }

            // HANDLE NON MULTIPLE CHOICE

            let question = game_state.current_question.clone();
            let possible_answers = question.answers.clone();

            let mut has_correct_continuation = false;
            let mut minimal_edit_dist = 10000;
            let mut truncate_amt = 0;

            let mut can_be_afm = false;
            let mut target_length = 0;

            for ans in possible_answers {
                // evaluate the answer
                let (truncate_amt_local, dist) = optimal_truncated_dist(guess, ans);

                if dist <= MAX_ACCEPTABLE_DIST {
                    // the guess is close enough
                    has_correct_continuation = true;
                    if dist < minimal_edit_dist
                        || dist == minimal_edit_dist && truncate_amt_local < truncate_amt
                    {
                        // we want to save the minimal edit distance, but if two possible answers have the same edit distance,
                        // we should pick the one which requires truncating the guess by fewer characters.
                        closest_answer = ans;
                        minimal_edit_dist = dist;
                        truncate_amt = truncate_amt_local;
                    }
                }
                if is_afm(ans, guess) {
                    // this is a possible AFM
                    can_be_afm = true;
                    target_length = ans.len();
                }
            }

            if !has_correct_continuation && can_be_afm {
                let res = GuessResultPublic {
                    game_state: game_state.into_public(id),
                    guess_res: GuessResult::AFM {
                        target_length,
                        guess_length: guess.chars().count(),
                    },
                };
                return serde_json::to_string(&res).unwrap();
            }

            let mut maybe_new_lifeline = None;
            let mut new_game_state = game_state.clone();

            if has_correct_continuation {
                // the user got the guess right
                let points_earned = if minimal_edit_dist != 0 {
                    // The guess was correct but not perfect.
                    if rand::thread_rng().gen_range(0..MAX_ACCEPTABLE_DIST) > minimal_edit_dist {
                        maybe_new_lifeline = Some(Lifeline::random_lifeline());
                    }
                    (MAX_ACCEPTABLE_DIST - minimal_edit_dist + 1) as i32
                } else {
                    // perfect match
                    maybe_new_lifeline = Some(Lifeline::random_lifeline());
                    POINTS_FOR_PERFECT_MATCH
                };

                new_game_state.score += points_earned;
                new_game_state.completed_question = true;
                if let Some(new_lifeline) = &maybe_new_lifeline {
                    new_game_state.lifeline_inv.add_lifeline(new_lifeline);
                }
                (*guard).insert(id.clone(), new_game_state.clone());

                let res = GuessResultPublic {
                    game_state: new_game_state.into_public_with_answers(id.clone()),
                    guess_res: GuessResult::Correct {
                        points_earned,
                        user_guess: guess.to_owned(),
                        answer: closest_answer.to_owned(),
                        new_lifeline: maybe_new_lifeline,
                    },
                };
                outer_game_state = new_game_state.clone();
                break 'outer_block res;
            } else {
                // The user has guessed wrong and the game is now over
                new_game_state.terminated = true;
                new_game_state.completed_question = true;
                (*guard).remove(&id);

                let res = GuessResultPublic {
                    game_state: new_game_state.into_public_with_answers(id.clone()),
                    guess_res: GuessResult::Incorrect {
                        user_guess: guess.to_owned(),
                        answer: closest_answer.to_owned(),
                    },
                };
                outer_game_state = new_game_state.clone();
                break 'outer_block res;
            }
        }
        return "{}".to_owned();
    };

    let gs = outer_game_state;

    let num_points_earned;
    let is_correct;
    let mut lifeline_earned: Option<String> = None;
    match &guess_res.guess_res {
        GuessResult::Correct {
            points_earned,
            new_lifeline,
            ..
        } => {
            num_points_earned = *points_earned;
            is_correct = true;
            lifeline_earned = new_lifeline.as_ref().map(|lifeline| lifeline.as_string());
        }
        GuessResult::Incorrect { .. } => {
            num_points_earned = 0;
            is_correct = false;
        }
        GuessResult::AFM { .. } => {
            unreachable!()
        }
    }
    // If the code runs to this point, then the guess is either correct or incorrect (not AFM state).
    // We can now record the guess into the database before returning.

    let _ = sqlx::query("INSERT INTO guesses VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW())")
        .bind(id.clone())
        .bind(gs.guesses_made)
        .bind(gs.current_question.song.album)
        .bind(gs.current_question.song.name)
        .bind(gs.current_question.shown_line)
        .bind(closest_answer)
        .bind(if is_correct { "correct" } else { "incorrect" })
        .bind(guess)
        .bind(num_points_earned)
        .bind(lifeline_earned)
        .bind(sqlx::types::Json(
            gs.hints_shown
                .iter()
                .map(|hint| hint.underlying_lifeline().as_string())
                .collect::<Vec<String>>(),
        ))
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
		",
        )
        .bind(gs.score)
        .bind(id.clone())
        .fetch_all(pool.inner())
        .await;
    }

    serde_json::to_string(&guess_res).unwrap()
}

fn is_afm(ans: &str, guess: &str) -> bool {
    if guess.chars().count() >= ans.chars().count() {
        return false;
    }
    let n = guess.chars().count();
    let ans = ans.chars().take(n).collect::<String>();

    lowercase_ignore_punctuation_edit_dist(&ans, guess) <= (n / 5)
}

/// whether `guess` is on the right track to `answer`. If a guess is on the right track,
/// then the player will be prompted to guess again without penalty.
///
/// Note that as implemented, the guess "owuefh" is on the right track to any guess, since
/// the after truncation of answer, the distance of truncated answer and guess will not exceed
/// `MAX_ACCEPTABLE_DIST`.
pub fn is_on_right_track(guess: &str, answer: &str) -> bool {
    let (_, dist) = optimal_truncated_dist(answer, guess);
    dist <= MAX_ACCEPTABLE_DIST
}

fn get_previous_lines(question: &Question) -> (String, bool) {
    const PREV_LINES_TO_SHOW: usize = 2;

    let preferred_answer = *question.answers.first().unwrap();

    let lines = &question.song.lines;
    let mut answer_position: usize = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.text == question.shown_line
            && index < lines.len() - 1
            && lines[index + 1].text == preferred_answer
        {
            answer_position = index;
            break;
        }
    }
    let mut output = String::new();

    let is_at_song_beginning = answer_position <= PREV_LINES_TO_SHOW;
    let beginning_index = std::cmp::max(answer_position as i32 - PREV_LINES_TO_SHOW as i32, 0);

    for line in lines
        .iter()
        .take(answer_position + 1)
        .skip(beginning_index as usize)
    {
        output.push_str(&format!("{}\n", line.text));
    }
    (output, is_at_song_beginning)
}
