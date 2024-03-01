use crate::song::{Line, Song};
use edit_distance::edit_distance;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Serialize;

/// If there are 16 distractors, then there are 17 answer choices in total.
const NUM_DISTRACTORS: i32 = 16;

/// A struct representing a question asked to the player.
/// The field `answer` is a list of possible answers, and
/// The field `song` is the song that the question
/// comes from, so these fields should be hidden from the user until the question is answered.
#[derive(Debug, Clone, Serialize)]
pub struct Question {
    pub shown_line: String,
    pub song: Song,
    pub answers: Vec<String>,
}

impl Question {
    /// Hides `answer` and `song`
    pub fn hide_answer_and_song(&self) -> Question {
        Question {
            shown_line: self.shown_line.clone(),
            song: Song {
                album: String::new(),
                name: String::new(),
                lyrics_raw: String::new(),
                lines: vec![],
            },
            answers: Vec::new(),
        }
    }
}

/// case insensitive edit distance
fn lowercase_edit_dist(a: &str, b: &str) -> usize {
    edit_distance(&a.to_lowercase(), &b.to_lowercase())
}

/// Generate the optimal truncation amount `x` of `l1` to minimize the edit distance between strings
/// `l1[..(l1.len() - x)]` and `l2`.
///
/// This function is needed because for each guess a player submits, we calculate the optimal truncation
/// amount, with the motivation being not to punish players who enter too much text.
pub fn optimal_truncated_dist(l1: &str, l2: &str) -> (i32, usize) {
    // first we will see how many characters we can truncate from the end of the guess to minimize lowercase_edit_dist(userGuess, answer)

    let mut optimal_k = 0;
    let mut minimal_dist = lowercase_edit_dist(l1, l2);
    let mut k: i32 = 1;
    while l1.len() as i32 - k >= 0 {
        // This part is needed so that k always ends on a character boundary, since slices operate on bytes, not characters.
        while l1.len() as i32 - k >= 0 && !l1.is_char_boundary(l1.len() - k as usize) {
            k += 1;
        }
        let d = lowercase_edit_dist(&l1[..(l1.len() - k as usize)], l2);
        if d < minimal_dist {
            optimal_k = k;
            minimal_dist = d;
        }
        k += 1;
    }
    (optimal_k, minimal_dist)
}

fn are_close_enough(s1: &str, s2: &str) -> bool {
    (edit_distance(s1, s2) as f32 / std::cmp::min(s1.chars().count(), s2.chars().count()) as f32)
        < 0.1_f32
}

fn is_acceptable_guess(guess: &Line) -> bool {
    guess.is_bad_prompt.is_none()
}

/// Generates distractor answer choices for a multiple choice question, while ensuring that
/// the distractors are not too close to the correct answer
///
pub fn pick_distractors(correct_answers: Vec<String>, songs: &Vec<Song>) -> Vec<String> {
    let mut distractors = Vec::new();
    for _ in 0..NUM_DISTRACTORS {
        let random_song = songs.choose(&mut rand::thread_rng()).unwrap();
        let mut random_line = random_song
            .lines
            .choose(&mut rand::thread_rng())
            .unwrap()
            .text
            .as_str();
        loop {
            let mut is_far_from_all_answers = true;
            for ans in &correct_answers {
                if are_close_enough(random_line, ans) {
                    is_far_from_all_answers = false;
                }
            }
            if is_far_from_all_answers {
                break;
            }

            random_line = random_song
                .lines
                .choose(&mut rand::thread_rng())
                .unwrap()
                .text
                .as_str();
        }
        distractors.push(random_line.to_owned());
    }

    // This is an easter egg, where there's a small probability for one of the distractors to be a funny quote by Ms. Swift:
    if rand::thread_rng().gen::<i32>() % 100 == 0 {
        distractors[0] = "umm I think for me...".to_owned();
    }

    distractors
}

/// Pick a random question from `songs_to_include`
pub fn pick_random_guess(songs: &[Song], songs_to_include: &[(String, String)]) -> Question {
    let songs: Vec<Song> = songs
        .iter()
        .filter(|song| songs_to_include.contains(&(song.album.clone(), song.name.clone())))
        .cloned()
        .collect();
    let random_song = songs.choose(&mut rand::thread_rng()).unwrap();

    let candidate_lines = &random_song.lines[0..(random_song.lines.len() - 1)];

    let mut random_line = candidate_lines.choose(&mut rand::thread_rng()).unwrap();
    while !is_acceptable_guess(random_line) {
        random_line = candidate_lines.choose(&mut rand::thread_rng()).unwrap();
    }

    let lines = &random_song.lines;
    let mut answers = Vec::new();
    for index in 0..(lines.len() - 1) {
        let next_line = &lines[index + 1].text;
        if lines[index].text == random_line.text && !answers.contains(next_line) {
            answers.push(next_line.clone());
        }
    }

    let q = Question {
        shown_line: random_line.text.clone(),
        answers,
        song: random_song.clone(),
    };
    dbg!(&q.answers);
    q
}
