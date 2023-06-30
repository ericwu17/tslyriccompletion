use include_dir::{include_dir, Dir};
use crate::song::Song;

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../lyrics_data");

pub fn load_songs_from_files() -> Vec<Song> {
  let mut songs: Vec<Song> = Vec::new();

  for album_dir in PROJECT_DIR.dirs() {
    for song_file in album_dir.files() {

      // Note: the directory names and file names have no effect on the underlying data,
      // except for changing the order in which the songs and albums will be loaded (and thus displayed)

      // each song file always follows the format of:
      // first line: contains album name
      // second line: contains song name
      // remaining lines: contains raw lyrics data


      let file_contents = song_file.contents_utf8().unwrap();
      let album_name_length = file_contents.find('\n').unwrap();
      
      let album_name = &file_contents[..album_name_length];
      let file_contents = &file_contents[album_name_length+1..];

      let song_name_length = file_contents.find('\n').unwrap();

      let song_name = &file_contents[..song_name_length];
      let file_contents = &file_contents[song_name_length+1..];

      songs.push(Song::new(album_name.to_owned(), song_name.to_owned(), file_contents.to_owned()));
    }
  }

  songs
}


