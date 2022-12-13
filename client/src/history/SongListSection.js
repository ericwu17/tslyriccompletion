import React from "react";
import { Box, Grid, Typography, Link, CircularProgress, Checkbox } from "@mui/material";
import axios from "axios";
import { ALBUM_ORDER, ALBUM_LOGOS, getAlbumChipWidth, generateSongHref } from "../utils/Utils";
import { Item } from "../song/SongPage";


export default function SongListSection({selectedSongs}) {
  const [fullSongList, setFullSongList] = React.useState({});
  React.useEffect(() => {
    axios.get("/songs").then((response) => {
      setFullSongList(response.data);
    });
  }, []);

  const albumChipWidth = getAlbumChipWidth();


  if (JSON.stringify(fullSongList) === "{}") {
    return (
      <CircularProgress />
    );
  }

  return (
    <>
      <Box mt={1} mb={1}>
        <Typography>
          <strong>
            This game was played with the following songs:
          </strong>
        </Typography>
      </Box>
      <Grid container rowSpacing={1} columnSpacing={{ xs: 1, sm: 2, md: 3 }}>
        {ALBUM_ORDER.map(album => {
          let songs = fullSongList[album];
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
                  <Box
                    key={index}
                    mb={-2}
                  >
                    <Typography textAlign="left" noWrap>
                      <Checkbox
                        checked={selectedSongs.filter(
                          elem => elem[0] === album && elem[1] === song)
                          .length > 0
                        }
                      />
                      <Link
                        href={generateSongHref(album, song)}
                        key={generateSongHref(album, song)}
                        target="_blank"
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
    </>
  );

}
