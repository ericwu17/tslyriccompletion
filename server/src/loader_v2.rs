//! The 2nd version of a loader, responsible for loading all Taylor Swift songs into memory
//! when the server starts.
//!
//! The original v1 of this loader used to load songs from a `taylor` submodule in the repository.
//! The v2 loader loads songs from the `lyrics_data` directory in the directory. This allows
//! me to have more control over the format of the songs, and which songs are considered part of the
//! game can be changed by directly adding/removing files.

use crate::song::Song;
use include_dir::{include_dir, Dir};
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

            let song = Song::new(
                album_name.to_owned(),
                song_name.to_owned(),
                file_contents.to_owned(),
            );

            // songs.push(song);

            let path = Path::new("../lyrics_data_new").join(song_file.path());
            let mut file = File::create(path).unwrap();
            file.write(album_name.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
            file.write(song_name.as_bytes()).unwrap();
            file.write(b"\n").unwrap();

            let mut raw_lines = file_contents.split('\n').collect::<VecDeque<_>>();
            dbg!(file_contents);

            for line in song.lines {
                while *raw_lines.front().unwrap() != line.text.as_str() {
                    let raw_line = raw_lines.pop_front().unwrap();
                    dbg!(raw_line);
                    if raw_line.starts_with('[') {
                        println!("writing raw line {}", raw_line);
                        file.write(raw_line.as_bytes()).unwrap();
                        file.write(b"\n").unwrap();
                    }
                }

                file.write(line.text.as_bytes()).unwrap();

                if line.is_exclamatory || line.has_bad_successor || line.has_multiple_successors {
                    file.write(b"$").unwrap();
                    if line.is_exclamatory {
                        file.write(b"<exclamatory>").unwrap();
                    }
                    if line.has_bad_successor {
                        file.write(b"<bad_succ>").unwrap();
                    }
                    if line.has_multiple_successors {
                        file.write(b"<mult_succ>").unwrap();
                    }
                }

                file.write(b"\n").unwrap();
            }
        }
    }

    songs
}
