import React from "react";
import axios from "axios";
import Cookies from "js-cookie";
import {
  Checkbox, Box, Grid, Button,
  Typography, Paper, Link, Alert, Snackbar,
  Dialog, DialogTitle, DialogActions, DialogContent, TextField
} from "@mui/material";
import { styled } from "@mui/material/styles";
import GameStateDisplay from "./GameStateDisplay";
import { ALBUM_LOGOS, ALBUM_ORDER, generateSongHref, getAlbumChipWidth } from "../utils/Utils";


const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#E0FFFF",
  ...theme.typography.body2,
  textAlign: "center",
  color: theme.palette.text.secondary,
}));


export default function Game() {
  const [hasStarted, setHasStarted] = React.useState(false);
  const [gameState, setGameState] = React.useState({});
  const [songList, setSongList] = React.useState({});
  const albumChipWidth = getAlbumChipWidth();

  const beginGame = () => {
    // if (!hasStarted) {
    //   return;
    // }
    const includedSongList = [];
    for (let album of Object.keys(songList)) {
      for (let name of Object.keys(songList[album])) {
        if (songList[album][name]) {
          includedSongList.push([album, name]);
        }
      }
    }

    axios.post("/game/start", includedSongList).then((response) => {
      setGameState(response.data);
      Cookies.set("tsgg-game-id", response.data.id);
      // eslint-disable-next-line no-console
      console.log("starting game with id: ", response.data.id);
      setHasStarted(true);
    });
  };

  React.useEffect(() => {
    axios.get("/songs").then((response) => {
      let songs = response.data;
      for (let album of Object.keys(songs)) {
        const names = [...songs[album]];
        songs[album] = {};
        for (let name of names) {
          songs[album][name] = true;
        }
      }
      setSongList(songs);
    });

    let maybeGameId = Cookies.get("tsgg-game-id");
    if (maybeGameId) {
      // eslint-disable-next-line no-console
      console.log(`Loading game from cookies... (the id is: ${maybeGameId})`);
      axios.get("/songs").then((response) => {
        const songs = response.data;
        axios.get(`/game/next?id=${maybeGameId}`).then((response) => {
          if (!response.data.id) {
            // return early because the game id in cookies is invalid.
            // eslint-disable-next-line no-console
            console.log("invalid id from cookies. Aborting.");
            return;
          }
          setGameState(response.data);
          // set the songList according to the songList in the current game
          const newSongList = {};

          for (let album of Object.keys(songs)) {
            const names = [...songs[album]];
            newSongList[album] = {};
            for (let name of names) {
              newSongList[album][name] = false;
            }
          }
          for (let [album, name] of response.data.included_songs) {
            newSongList[album][name] = true;
          }

          setSongList(newSongList);

          setHasStarted(true);
        });
      });

      return;
    }
  }, []);


  if (!hasStarted) {
    return (
      <Box m={2}>
        <Typography variant="h3">
          Are you ... Ready For It?
        </Typography>
        <Typography variant="body1">
          Select which songs you want to be quizzed on using the checkbox menu below.
          When you're ready, click "Begin"!
        </Typography>
        <Box sx={{width: "max-content", border: "2px solid green"}}>
          <Button onClick={beginGame} size="large">
            Begin
          </Button>
        </Box>
        <SongSelection
          songList={songList}
          setSongList={setSongList}
          albumChipWidth={albumChipWidth}
        />
      </Box>
    );
  } else {
    return <GameStateDisplay
      gameState={gameState}
      setGameState={setGameState}
      setHasStarted={setHasStarted}
    />;
  }
}

function SongSelection({songList, setSongList, albumChipWidth}) {

  const toggleSong = (album, song) => {
    const currentStatus = songList[album][song];
    setSongList({...songList, [album]: {...songList[album], [song]: !currentStatus}});
  };

  const [snackBarIsOpen, setSnackBarIsOpen] = React.useState(false);
  const [importDialogOpen, setImportDialogOpen] = React.useState(false);

  const hasAllSelected = album => {
    if (!songList[album]) {
      return false;
    }
    for (let song of Object.keys(songList[album])) {
      if (!songList[album][song]) {
        return false;
      }
    }
    return true;
  };

  const toggleAlbum = album => {
    let res;
    if (hasAllSelected(album)) {
      res = false;
    } else {
      res = true;
    }
    let newAlbum = {};
    for (let song of Object.keys(songList[album])) {
      newAlbum[song] = res;
    }
    setSongList({...songList, [album]: newAlbum});
  };
  const isAllSongsSelected = ALBUM_ORDER.map(album => hasAllSelected(album)).every(x => x);


  const toggleAll = () => {
    const res = isAllSongsSelected ? false : true;

    const newSongList = {};
    for (let album of ALBUM_ORDER) {
      const newAlbum = {};
      for (let song of Object.keys(songList[album])) {
        newAlbum[song] = res;
      }
      newSongList[album] = newAlbum;
    }
    setSongList(newSongList);
  };

  const invertSelection = () => {
    const newSongList = {};
    for (let album of ALBUM_ORDER) {
      const newAlbum = {};
      for (let song of Object.keys(songList[album])) {
        newAlbum[song] = !songList[album][song];
      }
      newSongList[album] = newAlbum;
    }
    setSongList(newSongList);
  };

  const copySelectedToClipboard = () => {
    const selectedSongString = getSelectedSongString(songList);
    navigator.clipboard.writeText(selectedSongString);
    setSnackBarIsOpen(true);
  };

  return (
    <Box sx={{ width: "100%" }}>
      <Box onClick={copySelectedToClipboard}>
        <Button>Copy selected songs to clipboard</Button>
      </Box>
      <Box>
        <Button onClick={() => setImportDialogOpen(true)}>Import selection</Button>
      </Box>
      <Snackbar
        open={snackBarIsOpen}
        autoHideDuration={6000}
        sx={{ border: "2px solid black" }}
        onClose={() => setSnackBarIsOpen(false)}
      >
        <Alert
          severity="success"
          sx={{ width: "100%" }}
          onClose={() => setSnackBarIsOpen(false)}
        >
          Successfully copied to clipboard
        </Alert>
      </Snackbar>
      <ImportSongStringDialog
        open={importDialogOpen}
        onClose={() => setImportDialogOpen(false)}
        setSongList={setSongList}
        songList={songList}
      />
      <Box>
        <Button onClick={toggleAll}>{isAllSongsSelected ? "Deselect" : "Select"} all</Button>
        <Button onClick={invertSelection}>Invert Selection</Button>
      </Box>
      <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
        {ALBUM_ORDER.map(album => {
          let songs = songList[album];
          return (
            <Grid item xs={albumChipWidth} key={album}>
              <Item sx={{height: "100%", mb: 1, p: 1}}>
                <Box display="flex" justifyContent="center" alignItems="center" width="100%">
                  <Checkbox checked={hasAllSelected(album)} onClick={() => toggleAlbum(album)} />
                  <Typography variant="h4" noWrap>
                    {album}
                  </Typography>
                  <Box
                    component="img"
                    sx={{
                      height: 50,
                      width: 50,
                    }}
                    alt="Album Img"
                    src={ALBUM_LOGOS[album]}
                    ml={1}
                  />
                </Box>
                {songs && Object.keys(songs).map((song, index) =>
                  <Box
                    display="flex"
                    flexDirection="row"
                    alignItems="center"
                    key={`/song/${album}/${song}`}
                    mb={-2}
                  >
                    <Checkbox
                      checked={songList[album][song]}
                      onClick={() => toggleSong(album, song)}
                    />
                    <Typography noWrap>
                      {index+1}) {}
                      <Link href={generateSongHref(album, song)} >
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
    </Box>
  );
}

function getSelectedSongString(songList) {
  var result = [];
  for (let album of ALBUM_ORDER) {
    for (let song of Object.keys(songList[album])) {
      if (songList[album][song]) {
        result.push(song);
      }
    }
  }

  const resultString = result.join("\n");
  return resultString;
}





function ImportSongStringDialog({open, onClose, setSongList, songList}) {
  const [currInputStr, setCurrInputStr] = React.useState("");

  const handleCloseDialog = () => {
    const selectedSongsArr = currInputStr.split("\n");
    for (let album of ALBUM_ORDER) {
      for (let song of Object.keys(songList[album])) {
        songList[album][song] = selectedSongsArr.includes(song);
      }
    }
    setSongList(songList);

    onClose();
  };

  return (
    <Dialog
      onClose={handleCloseDialog}
      open={open}
    >
      <DialogTitle id="customized-dialog-title" onClose={handleCloseDialog}>
        Please paste selected songs here
      </DialogTitle>
      <DialogContent dividers>
        <TextField
          style={{textAlign: "left"}}
          size="small"
          multiline
          onChange={event => setCurrInputStr(event.target.value)}
          value={currInputStr}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={handleCloseDialog}>
          Save Changes
        </Button>
      </DialogActions>
    </Dialog>
  );
}