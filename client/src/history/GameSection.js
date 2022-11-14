import React from "react";
import { Box } from "@mui/material";

import { parseISO } from "date-fns";

export default function GameSection({game}) {
  console.log(game);
  const name = game.player_name || "<Anonymous>";

  const score = game.terminal_score;

  const startTime = parseISO(game.start_time);

  return (
    <Box m={2} display="flex">
      {startTime.toLocaleString()}, Played by {name}. Score: {score}
    </Box>
  );
}