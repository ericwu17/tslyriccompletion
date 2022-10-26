import { useParams } from "react-router-dom";
import axios from 'axios';
import React from "react";
import {Tooltip, Typography, Box, Grid, Paper, Link} from '@mui/material';
import { styled } from '@mui/material/styles';
import { ALBUM_ORDER } from "../utils/Utils";


const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#E0FFFF',
  ...theme.typography.body2,
  textAlign: 'center',
  color: theme.palette.text.secondary,
}));


export default function SongPage() {
  let { album, name } = useParams();

  const [songList, setSongList] = React.useState({});
  const [song, setSong] = React.useState({});

  React.useEffect(() => {
    axios.get(`/songs`).then((response) => {
      setSongList(response.data);
    });
  }, [])
  React.useEffect(() => {
    axios.get(`/songs/${album}/${name}`).then((response) => {
      setSong(response.data);
    })
  }, [album, name])


  if (album === undefined) {
    return (
      <Box sx={{ width: '100%' }}>
        <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
          {ALBUM_ORDER.map(album => {
            let songs = songList[album];
            return (
              <Grid item xs={4} key={album}>
                <Item sx={{height: '100%', m: 2, p: 2}}>
                  <Typography variant="h4">{album}</Typography>
                  {songs && songs.map((song, index) => 
                    <Typography>
                      {index+1}) <Link href={`/song/${album}/${song}`} key={`/song/${album}/${song}`}>{song}
                      </Link>
                    </Typography>
                  )}
                </Item>
              </Grid>
            );
          })}
        </Grid>
      </Box>
    );
  }
  if (album !== undefined && name !== undefined && song.lyrics_raw) {
    return displaySong(song);
  }


  function displaySong(song) {
    let lines = song.lyrics_raw.split('\n');
    let lineInfos = song.lines;

    let renderedLines = [];
    for (let lineInfo of lineInfos) {
      while (lineInfo.text !== lines[0]) {
        renderedLines.push(<Typography sx={{ fontWeight: 'bold' }}>{lines.shift()}</Typography>)
      }
      if (lineInfo.has_bad_successor || lineInfo.has_multiple_successors || lineInfo.is_exclamatory) {
        let tooltipText = "";
        if (lineInfo.has_bad_successor) {
          tooltipText += "Has a bad successor\n"
        }
        if (lineInfo.has_multiple_successors) {
          tooltipText += "Has multiple different successors\n"
        }
        if (lineInfo.is_exclamatory) {
          tooltipText += "Is an exclamatory line"
        }

        renderedLines.push(
          <Tooltip title={
            <div style={{ whiteSpace: 'pre-line' }}>{tooltipText}</div>
          } placement="right">
            <Typography color="pink" sx={{
              width: 'max-content',
            }}>
              {lines.shift()}
            </Typography>
          </Tooltip>
        );
      } else {
        renderedLines.push(<Typography>{lines.shift()}</Typography>)
      }
    }

    return (
      <Box mt={2} ml={5} mb={30}>
        <Typography variant="h4">{song.album} : {song.name}</Typography>
        {renderedLines}
      </Box>
    )
  }

  return (
    <>
      <div>
        This is the song page!
      </div>
      <div>
        <h2>{album} -- {name}</h2>
      </div>
    </>
  );
}


