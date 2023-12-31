use colored::Colorize;
use rand::prelude::*;
use serde::Serialize;
use std::fmt;

/// An enum representing one of three possible lifelines.
#[derive(Serialize, Clone)]
pub enum Lifeline {
    ShowTitleAlbum,
    ShowPrevLines,
    Skip,
}

impl Lifeline {
    pub fn random_lifeline() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..3) {
            0 => Lifeline::ShowTitleAlbum,
            1 => Lifeline::ShowPrevLines,
            _ => Lifeline::Skip,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Lifeline::ShowPrevLines => "ShowPrevLines".to_owned(),
            Lifeline::ShowTitleAlbum => "ShowTitleAlbum".to_owned(),
            Lifeline::Skip => "Skip".to_owned(),
        }
    }
}

/// A struct representing how many of each lifeline a player has available.
#[derive(Serialize, Clone, Debug)]
pub struct LifelineInventory {
    show_title_album: i32,
    show_prev_lines: i32,
    skip: i32,
}

impl LifelineInventory {
    pub fn new() -> LifelineInventory {
        LifelineInventory {
            show_title_album: 1,
            show_prev_lines: 1,
            skip: 1,
        }
    }
    pub fn consume_lifeline(&mut self, lifeline: Lifeline) -> bool {
        match lifeline {
            Lifeline::ShowTitleAlbum => {
                if self.show_title_album > 0 {
                    self.show_title_album -= 1;
                    return true;
                }
                false
            }
            Lifeline::ShowPrevLines => {
                if self.show_prev_lines > 0 {
                    self.show_prev_lines -= 1;
                    return true;
                }
                false
            }
            Lifeline::Skip => {
                if self.skip > 0 {
                    self.skip -= 1;
                    return true;
                }
                false
            }
        }
    }
    pub fn add_lifeline(&mut self, lifeline: &Lifeline) {
        match lifeline {
            Lifeline::ShowTitleAlbum => {
                self.show_title_album += 1;
            }
            Lifeline::ShowPrevLines => {
                self.show_prev_lines += 1;
            }
            Lifeline::Skip => {
                self.skip += 1;
            }
        }
    }
}

impl Default for LifelineInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LifelineInventory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "You currently have:\n\tShow Title Lifelines (?t): {}\n\tShow Previous Lines Lifelines (?p): {}\n\tSkip Question Lifelines (?s): {}", 
			self.show_title_album.to_string().red().bold(),
			self.show_prev_lines.to_string().red().bold(),
			self.skip.to_string().red().bold(),
		)
    }
}

impl fmt::Display for Lifeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Lifeline::ShowPrevLines => {
                write!(f, "{} Lifeline", "Show Previous Lines".bold())
            }
            Lifeline::ShowTitleAlbum => {
                write!(f, "{} Lifeline", "Show Title".bold())
            }
            Lifeline::Skip => {
                write!(f, "{} Lifeline", "Skip Question".bold())
            }
        }
    }
}
