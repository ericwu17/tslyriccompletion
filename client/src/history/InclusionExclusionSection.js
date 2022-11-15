import React from "react";
import { Box} from "@mui/material";

export default function InclusionExclusionSection({selectedSongs}) {
  if (selectedSongs.Exclude) {
    const excluded = selectedSongs.Exclude;
    if (excluded.length === 0) {
      return (
        <Box>
          This game was played with all songs.
        </Box>
      );
    }
    return (
      <>
        <Box>
          This game was played with all songs except for:
        </Box>
        <SongSetList songSet={excluded}/>
      </>
    );
  } else if (selectedSongs.Include) {
    const included = selectedSongs.Include;
    if (included.length === 0) {
      return (
        <Box>
          This game was played with no songs at all (this shouldn't be possible and if you're seeing
          this it means there's been an error).
        </Box>
      );
    }
    return (
      <>
        <Box>
          This game was played with only the following songs:
        </Box>
        <SongSetList songSet={included}/>
      </>
    );
  }



  return (
    <Box>
      There was an error processing the Inclusion/Exclusion list of the game :(
    </Box>
  );

}


function SongSetList({songSet}) {
  const songs = songSet.filter(object => object.Song).map(object => object.Song);
  const albums = songSet.filter(object => object.Album).map(object => object.Album);
  return (
    <>
      <Box display="flex" flexDirection="column">
        {albums.map(album => (
          <Box key={album}>
            • The entire album: {album}
          </Box>
        ))}
        {songs.map(song => (
          <Box key={song}>
            • {song[0]} -- {song[1]}
          </Box>
        ))}
      </Box>
    </>
  );
}