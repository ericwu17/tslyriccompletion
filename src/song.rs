
#[derive(Debug)]
pub struct Song {
	pub album: String,
	pub name: String,
	pub lyrics_raw: String,
	pub lines: Vec<String>,
}

impl Song {
	pub fn new(album: String, name: String, lyrics_raw: String) -> Self {
		let v: Vec<String> = lyrics_raw.split("\n").filter(|x| !(x.starts_with("[") || x == &"")).map(|x| x.trim().to_owned()).collect();
		
		Song {
			album, name, lyrics_raw, lines: v,
		}
	}
}