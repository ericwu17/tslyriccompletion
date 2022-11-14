import axios from "axios";
import React from "react";
import { useSearchParamsState } from "../utils/Utils";
import { parseISO } from "date-fns";
import { Box, Table, TableRow, TableCell, TableBody, CircularProgress } from "@mui/material";
import InclusionExclusionSection from "./InclusionExclusionSection";

export default function GameDetails() {
  const [data, setData] = React.useState({});
  const [fetchError, setFetchError] = React.useState(false);
  const id = useSearchParamsState("id", "")[0];

  React.useEffect(() => {
    axios.get(`/history/game?id=${id}`).then((response) => {
      setData(response.data);
      if (response.status !== 200) {
        setFetchError(true);
      }
    }).catch(function() {
      setFetchError(true);
    });
  }, []);

  if (fetchError) {
    return (
      <div>
        There was an error fetching the content! Please check the "id" value in the url.
        If you think this is a bug, please let me (Eric) know.
      </div>
    );
  } else if (JSON.stringify(data) == "{}") {
    return (
      <CircularProgress />
    );
  }

  let {game, guesses} = data;

  const name = game.player_name || "<Anonymous>";

  const score = game.terminal_score;

  const startTime = parseISO(game.start_time);
  let numGuesses = guesses.length;


  return (
    <Box m={2} display="flex" flexDirection="column" alignItems="center">
      <Box sx={{border: "3px solid #B9D9EB", borderRadius: "5px"}} p={1}>
        <Table>
          <TableBody>
            <TableRow>
              <TableCell><strong>Start time: {}</strong></TableCell>
              <TableCell>{startTime.toLocaleString()}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Played By: {}</strong></TableCell>
              <TableCell>{name}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Final Score: {}</strong></TableCell>
              <TableCell>{score}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Number of Guesses: {}</strong></TableCell>
              <TableCell>{numGuesses}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell colSpan={2}>
                <InclusionExclusionSection selectedSongs={game.selected_songs}/>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </Box>
    </Box>
  );

}
