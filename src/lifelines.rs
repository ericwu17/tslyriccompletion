use std::fmt;

use colored::Colorize;

pub enum Lifeline {
	ShowTitleAlbum,
	ShowPrevLines,
	Skip,
}

pub struct LifelineInventory {
	show_title_album: i32,
	show_prev_lines: i32,
	skip: i32,
}

impl LifelineInventory {
	pub fn new() -> LifelineInventory {
		LifelineInventory { show_title_album: 1, show_prev_lines: 1, skip: 1 }
	}
	pub fn consume_lifeline(&mut self, lifeline: Lifeline) -> bool {
		match lifeline {
			Lifeline::ShowTitleAlbum => {
				if self.show_title_album > 0 {
					self.show_title_album -= 1;
					return true;
				}
				return false;
			}
			Lifeline::ShowPrevLines => {
				if self.show_prev_lines > 0 {
					self.show_prev_lines -= 1;
					return true;
				}
				return false;
			}
			Lifeline::Skip => {
				if self.skip > 0 {
					self.skip -= 1;
					return true;
				}
				return false;
			}
		}
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