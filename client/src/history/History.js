import React from "react";
import { Box, Typography } from "@mui/material";
import QueryMenuBar from "./QueryMenuBar";

export default function HistoryPage() {
  const [games, setGames] = React.useState([]);
  console.log(games);

  return (
    <Box m={2}>
      <QueryMenuBar setGames={setGames} />
      <Typography>
        This is the history page (coming soon)
      </Typography>
    </Box>
  );
}