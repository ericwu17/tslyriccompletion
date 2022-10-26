import { useParams } from "react-router-dom";
import axios from 'axios';
import React from "react";
import {Tooltip, Typography} from '@mui/material';


export default function SongPage() {
  let { album, name } = useParams();

  const [songList, setSongList] = React.useState({});
  const [song, setSong] = React.useState({});
  console.log(song);

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
    return Object.keys(songList).map(key => {

      return <AlbumSection key={key} albumTitle={key} songs={songList[key]}></AlbumSection>
    })
  }
  if (album !== undefined && name !== undefined && song.lyrics_raw) {
    return displaySong(song);
  }


  function displaySong(song) {
    let lines = song.lyrics_raw.split('\n');
    let lineInfos = song.lines;

    let renderedLines = [];
    for (let lineInfo of lineInfos) {
      debugger;
      while (lineInfo.text !== lines[0]) {
        renderedLines.push(<Typography>{lines.shift()}</Typography>)
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
            <Typography color="gray" sx={{
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
      <>
        <Typography variant="h4">{song.album} : {song.name}</Typography>
        {renderedLines}
      </>
    )
  }

  function AlbumSection({albumTitle, songs}) {
    return (
      <>
        <div>{albumTitle}</div>
        {songs.map(songName => 
          <a href={`/song/${albumTitle}/${songName}`} key={songName}>
            {songName}
          </a>
        )}
      </>
    );
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


