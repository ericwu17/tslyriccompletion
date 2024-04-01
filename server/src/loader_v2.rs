//! The 2nd version of a loader, responsible for loading all Taylor Swift songs into memory
//! when the server starts.
//!
//! The original v1 of this loader used to load songs from a `taylor` submodule in the repository.
//! The v2 loader loads songs from the `lyrics_data` directory in the directory. This allows
//! me to have more control over the format of the songs, and which songs are considered part of the
//! game can be changed by directly adding/removing files.

use crate::song::Song;
use include_dir::{include_dir, Dir};

/// Directory of lyrics files
static LYRICS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../lyrics_data");

/// Load songs from directory into a vector of [`Song`] structs.
pub fn load_songs_from_files() -> Vec<Song> {
    let mut songs: Vec<Song> = Vec::new();

    for album_dir in LYRICS_DIR.dirs() {
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
            let file_contents = &file_contents[album_name_length + 1..];

            let song_name_length = file_contents.find('\n').unwrap();

            let song_name = &file_contents[..song_name_length];
            let file_contents = &file_contents[song_name_length + 1..];

            songs.push(Song::new(
                Box::leak(album_name.to_owned().into_boxed_str()),
                Box::leak(song_name.to_owned().into_boxed_str()),
                Box::leak(file_contents.to_owned().into_boxed_str()),
            ));
        }
    }

    songs
}
