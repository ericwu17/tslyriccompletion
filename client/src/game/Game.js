import React from "react";
import axios from "axios";
import Cookies from "js-cookie";
import { Checkbox, Box, Grid, Button, Typography, Paper, Link } from "@mui/material";
import { styled } from "@mui/material/styles";
import GameStateDisplay from "./GameStateDisplay";
import { ALBUM_LOGOS, ALBUM_ORDER } from "../utils/Utils";


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
  const [flag, setFlag] = React.useState(0);

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

  React.useEffect(() => {
    const keyDownHandler = event => {
      if (event.key === "Enter") {
        event.preventDefault();
        // For some reason, if I call the function beginGame()
        // right here, it does not properly capture the current songList
        // (songList will be {} inside the function)
        // Hence I change an integer flag, and observe
        // changes to the flag in another useEffect.
        setFlag(17);
        document.removeEventListener("keydown", keyDownHandler);
      }
    };
    if (!hasStarted) {
      document.addEventListener("keydown", keyDownHandler);
      setFlag(0);
    } else {
      document.removeEventListener("keydown", keyDownHandler);
    }

    return () => {
      document.removeEventListener("keydown", keyDownHandler);
    };
  }, [hasStarted]);

  React.useEffect(() => {
    if (flag === 17) {
      // eslint-disable-next-line no-console
      console.log("calling beginGame due to Enter being pressed!");
      beginGame();
    }
  }, [flag]);


  if (!hasStarted) {
    return (
      <Box m={2}>
        <Typography variant="h3" sx={{textDecoration: "underline"}}>
          Are you ... Ready For It?
        </Typography>
        <Typography variant="body1">
          Select which songs you want to be quizzed on using the checkbox menu below.
          When you're ready, click "Begin"! (or press the enter key)
        </Typography>
        <Box sx={{width: "max-content", border: "2px solid green"}}>
          <Button onClick={beginGame} size="large">
            Begin
          </Button>
        </Box>
        <SongSelection songList={songList} setSongList={setSongList}/>
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

function SongSelection({songList, setSongList}) {

  const toggleSong = (album, song) => {
    const currentStatus = songList[album][song];
    setSongList({...songList, [album]: {...songList[album], [song]: !currentStatus}});
  };


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

  return (
    <Box sx={{ width: "100%" }}>
      <Button onClick={toggleAll}>{isAllSongsSelected ? "Deselect" : "Select"} all</Button>
      <Button onClick={invertSelection}>Invert Selection</Button>
      <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
        {ALBUM_ORDER.map(album => {
          let songs = songList[album];
          return (
            <Grid item xs={3} key={album}>
              <Item sx={{height: "100%", mb: 1, p: 1}}>
                <Box display="flex" justifyContent="center" alignItems="center" width="100%">
                  <Checkbox checked={hasAllSelected(album)} onClick={() => toggleAlbum(album)} />
                  <Typography variant="h4" sx={{textDecoration: "underline"}} noWrap>
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
                    key={`/tswift/song/${album}/${song}`}
                    mb={-2}
                  >
                    <Checkbox
                      checked={songList[album][song]}
                      onClick={() => toggleSong(album, song)}
                    />
                    <Typography noWrap>
                      {index+1}) {}
                      <Link href={`/tswift/song/${album}/${song}`} >
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

