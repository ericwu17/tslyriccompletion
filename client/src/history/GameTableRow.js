import React from "react";
import { TableRow, TableCell, Link, Box } from "@mui/material";

import { parseISO } from "date-fns";

export default function GameTableRow({game, index}) {
  const score = game.terminal_score;
  const { num_guesses } = game;

  const startTime = parseISO(game.start_time);

  return (
    <TableRow
      sx={{
        "&:last-child td, &:last-child th": { border: 0 },
        background: index %2 === 0 ? "white" : "WhiteSmoke",
      }}
    >
      <TableCell component="th" scope="row">
        {/* The first box has no padding on the left */}
        <Box>
          {startTime.toLocaleString()}
        </Box>
      </TableCell>
      <TableCell>
        <Box pl={1}>
          <PlayerNameDisplay name={game.player_name} username={game.username}/>
        </Box>
      </TableCell>
      <TableCell align="right">
        <Box pl={1}>
          {num_guesses}
        </Box>
      </TableCell>
      <TableCell align="right">
        <Box pl={1}>
          {score}
        </Box>
      </TableCell>
      <TableCell align="right">
        <Box pl={1}>
          <Link href={`/history/game?id=${game.uuid}`}>Details</Link>
        </Box>
      </TableCell>
    </TableRow>
  );
}

export function PlayerNameDisplay({name, username}) {
  if (username) {
    return (
      <span style={{color:"darkgreen", fontWeight: "bold"}}>{username}</span>
    );
  }
  if (name) {
    return (
      <span>{name}</span>
    );
  }

  return (
    <span style={{color:"darkgray", fontWeight: "bold"}}>{"<Anonymous>"}</span>
  );
}