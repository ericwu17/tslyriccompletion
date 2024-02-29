import { useParams } from "react-router-dom";
import axios from "axios";
import React from "react";
import {
  Tooltip, Typography, Box, Grid, Paper, Link, TextField, CircularProgress, Popover
} from "@mui/material";
import { styled } from "@mui/material/styles";
import {
  ALBUM_LOGOS, ALBUM_ORDER,
  generateLineHistoryHref,
  generateSongHref, getAlbumChipWidth, isTouchDevice, normalizeQuotes
} from "../utils/Utils";
import { LinePopoverContent } from "../history/GuessHistory";
import { NotFoundImg } from "../not-found/NotFound";

export const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#E0FFFF",
  ...theme.typography.body2,
  textAlign: "center",
  color: theme.palette.text.secondary,
}));


export default function SongPage() {
  let { album, name } = useParams();
  const albumChipWidth = getAlbumChipWidth();

  const [unfilteredSongList, setSongList] = React.useState({});
  const [song, setSong] = React.useState({});
  const [searchString, setSearchString] = React.useState("");
  const [isListLoading, setIsListLoading] = React.useState(true);
  const [isSongLoading, setIsSongLoading] = React.useState(true);

  const normalizedSearchString = normalizeQuotes(searchString).toLowerCase();

  React.useEffect(() => {
    setIsListLoading(true);
    axios.get("/songs").then((response) => {
      setSongList(response.data);
      setIsListLoading(false);
    });
  }, []);
  React.useEffect(() => {
    setIsSongLoading(true);
    axios.get(`/songs/${album}/${name}`).then((response) => {
      setSong(response.data);
      setIsSongLoading(false);
    });
  }, [album, name]);

  if (isListLoading || isSongLoading) {
    return (
      <CircularProgress />
    );
  }

  let songList = {};
  let shownSongsArr = [];
  for (let albumName of ALBUM_ORDER) {
    let songArr = unfilteredSongList[albumName].filter(
      song => song.toLowerCase().includes(normalizedSearchString)
    );
    songList[albumName] = songArr;
    shownSongsArr.push(...songArr.map(song => ({"album": albumName, "name": song})));
  }


  const onKeyDown = e => {
    if (e.key === "Enter" && shownSongsArr.length === 1) {
      const song = shownSongsArr[0];
      window.location.href = generateSongHref(song.album, song.name);
    }
  };

  if (album === undefined) {
    return (
      <Box m={2} maxWidth="100%">
        <Typography variant="h3">
          Browse Taylor Swift Lyrics
        </Typography>
        <TextField
          sx={{
            width:"100%",
            background:shownSongsArr.length === 1 ? "lightgreen" : null,
          }}
          placeholder="Search songs..."
          value={searchString}
          onChange={event => setSearchString(event.target.value)}
          autoFocus
          color={shownSongsArr.length === 1 ? "success" : "primary"}
          onKeyDown={onKeyDown}
          helperText={shownSongsArr.length === 1 && "Press enter to continue"}
        />
        <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
          {ALBUM_ORDER.map(album => {
            let songs = songList[album];
            if (songs.length === 0) {
              return null;
            }
            return (
              <Grid item xs={albumChipWidth} key={album}>
                <Item sx={{height: "100%", m: 0.5, p: 2}}>
                  <Box display="flex" justifyContent="center" alignItems="center" width="100%">
                    <Box
                      component="img"
                      sx={{
                        height: 50,
                        width: 50,
                      }}
                      alt="Album Img"
                      src={ALBUM_LOGOS[album]}
                      mr={1}
                    />
                    <Typography variant="h4" noWrap>
                      {album}
                    </Typography>
                  </Box>
                  {songs && songs.map((song, index) =>
                    <Box key={index}>
                      <Typography textAlign="left" noWrap>
                        {index+1}) {}
                        <Link
                          href={generateSongHref(album, song)}
                          key={generateSongHref(album, song)}
                        >
                          {song}
                        </Link>
                      </Typography>
                    </Box>
                  )}
                </Item>
              </Grid>
            );
          })}
        </Grid>
        {shownSongsArr.length === 0 &&
          <Box m={2}>
            <Typography variant="h5">No songs matched your search</Typography>
          </Box>
        }
      </Box>
    );
  }
  if (album !== undefined && name !== undefined && song.lyrics_raw) {
    return displaySong(song);
  }


  function displaySong(song) {
    let lines = song.lyrics_raw.split("\n");
    let lineInfos = song.lines;

    let renderedLines = [];
    for (let lineInfo of lineInfos) {
      while (lineInfo.text.trim() !== lines[0].trim()) {
        renderedLines.push(<Typography sx={{ fontWeight: "bold" }}>{lines.shift()}</Typography>);
      }
      if (lineInfo.is_bad_prompt != null) {
        let tooltipText = lineInfo.is_bad_prompt;
        renderedLines.push(
          <Tooltip title={
            <div style={{ whiteSpace: "pre-line" }}>{tooltipText}</div>
          } placement="right">
            <Typography color="#69b6cf" sx={{
              width: "fit-content",
            }}>
              {lines.shift()}
            </Typography>
          </Tooltip>
        );
      } else {
        renderedLines.push(
          <GuessableLine
            line={lines.shift()} song={name}
            album={album} numGuesses={lineInfo.num_guesses}
          />
        );
      }
    }

    return (
      <Box my={2} px={5} maxWidth="100%">
        <Typography variant="h4">{song.album} : {song.name}</Typography>
        {renderedLines}
      </Box>
    );
  }

  return (
    <Box
      mx={4} my={2}
      display="flex" flexDirection="column"
      alignItems="center"
    >
      <Typography>
        Error: song not found! If you're typing in the URL by hand,
        note that the characters must be an exact match (and it's case sensitive!)
      </Typography>
      <Typography>
        Album: {album}
      </Typography>
      <Typography>
        Name: {name}
      </Typography>
      <NotFoundImg />
    </Box>
  );
}

function GuessableLine({album, song, line, numGuesses}) {
  const [anchorEl, setAnchorEl] = React.useState(null);

  const handlePopoverOpen = event => {
    setAnchorEl(event.currentTarget);
  };

  const handlePopoverClose = () => {
    setAnchorEl(null);
  };
  // We never want to show the popover when the user is on a touchscreen device, because the idea
  // of the popover was to show info when the user MOUSE OVER the line. Users can't mouse over
  // lines when using a mobile device.
  const popoverOpen = Boolean(anchorEl) && !isTouchDevice();


  return (
    <Box
      aria-owns={open ? "mouse-over-popover" : undefined}
      aria-haspopup="true"
      onMouseEnter={handlePopoverOpen}
      onMouseLeave={handlePopoverClose}
      sx={{
        width: "fit-content",
      }}
    >
      <Typography>
        <Link
          underline="none" sx={{color:"black"}}
          href={generateLineHistoryHref(album, song, line)}
        >
          {line}<sub>{numGuesses}</sub>
        </Link>
      </Typography>
      <Popover
        sx={{
          pointerEvents: "none",
        }}
        open={popoverOpen}
        anchorEl={anchorEl}
        anchorOrigin={{
          vertical: "center",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "center",
          horizontal: "left",
        }}
        onClose={handlePopoverClose}
        disableScrollLock
      >
        <LinePopoverContent
          album={album}
          song={song}
          prompt={line}
        />
      </Popover>
    </Box>
  );
}

