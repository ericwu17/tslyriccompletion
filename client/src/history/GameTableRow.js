import React from "react";
import { TableRow, TableCell, Link } from "@mui/material";

import { parseISO } from "date-fns";

export default function GameTableRow({game, index}) {
  const name = game.player_name;

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
        {startTime.toLocaleString()}
      </TableCell>
      {name && <TableCell>{name}</TableCell>}
      {!name &&
        <TableCell>
          <span style={{color:"darkgray", fontWeight: "bold"}}>{"<Anonymous>"}</span>
        </TableCell>
      }
      <TableCell align="right">{num_guesses}</TableCell>
      <TableCell align="right">{score}</TableCell>
      <TableCell align="right">
        <Link href={`/history/game?id=${game.uuid}`}>See Details</Link>
      </TableCell>
    </TableRow>
  );
}