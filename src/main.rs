mod loader;
mod song;
mod guess_generating;
mod game;

use std::error::Error;
use song::Song;
use game::run_game_loop;


fn main() -> Result<(), Box<dyn Error>> {
	run_game_loop();

	Ok(())
}
