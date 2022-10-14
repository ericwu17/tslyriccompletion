mod loader;
mod song;
mod guess_generating;
mod game;
mod diff;

use std::error::Error;
use song::Song;
use game::{run_game_loop, input};
use diff::diff_greedy;

fn main() -> Result<(), Box<dyn Error>> {
	
	println!("{:?}", diff_greedy(&"Now all we know is don’t let go".to_lowercase(), &"You showed me colors you know I can't see with anyone else".to_lowercase()));
	input("");
	run_game_loop();

	Ok(())
}
