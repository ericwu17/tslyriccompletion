import React from "react";
import axios from "axios";
import { Checkbox, Box, Grid, Button, Typography, Paper, Link } from "@mui/material";
import { styled } from '@mui/material/styles';
import GameStateDisplay from "./GameStateDisplay";
import { ALBUM_LOGOS, ALBUM_ORDER } from "../utils/Utils";


const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#E0FFFF',
  ...theme.typography.body2,
  textAlign: 'center',
  color: theme.palette.text.secondary,
}));


export default function Game() {
  const [hasStarted, setHasStarted] = React.useState(false);
  const [gameState, setGameState] = React.useState({});
  const [songList, setSongList] = React.useState({});

  const beginGame = () => {
    const includedSongList = [];
    for (let album of Object.keys(songList)) {
      for (let name of Object.keys(songList[album])) {
        if (songList[album][name]) {
          includedSongList.push([album, name]);
        }
      }
    }

    axios.post(`/game/start`, includedSongList).then((response) => {
      setGameState(response.data);
      setHasStarted(true);
    })
    setHasStarted(true);
  }

  React.useEffect(() => {
    axios.get(`/songs`).then((response) => {
      let songs = response.data
      for (let album of Object.keys(songs)) {
        const names = [...songs[album]];
        songs[album] = {};
        for (let name of names) {
          songs[album][name] = false;
        }
      }
      setSongList(songs);
    });
  }, [])

  if (!hasStarted) {
    return (
      <>
        <SongSelection songList={songList} setSongList={setSongList}/>
        <Button onClick={beginGame}>
          Begin
        </Button>
      </>
    );
  } else {
    return <GameStateDisplay 
      gameState={gameState}
      setGameState={setGameState}
      setHasStarted={setHasStarted}
      restartGame={beginGame}
    />
  }
}

function SongSelection({songList, setSongList}) {

  const toggleSong = (album, song) => {
    const currentStatus = songList[album][song];
    setSongList({...songList, [album]: {...songList[album], [song]: !currentStatus}});
  }


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
  }

  return (
    <Box sx={{ width: '100%' }}>
      <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
        {ALBUM_ORDER.map(album => {
          let songs = songList[album];
          return (
            <Grid item xs={4} key={album}>
              <Item sx={{height: '100%', m: 2, p: 2}}>
                <Box display="flex" justifyContent="center" alignItems="center" width="100%">
                  <Checkbox checked={hasAllSelected(album)}/>
                  <Typography variant="h4" sx={{textDecoration: 'underline'}}>{album}</Typography>
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
                  <Box display="flex" flexDirection="row" alignItems="center" key={`/song/${album}/${song}`} mb={-2}>
                    <Checkbox checked={songList[album][song]} onClick={() => toggleSong(album, song)}/>
                    <Typography>
                      {index+1}) <Link href={`/song/${album}/${song}`} >{song}
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

