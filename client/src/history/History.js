import React from "react";
import { Box } from "@mui/material";
import QueryMenuBar from "./QueryMenuBar";
import GameSection from "./GameSection";

export default function HistoryPage() {
  const [games, setGames] = React.useState([]);

  return (
    <Box m={2}>
      <QueryMenuBar setGames={setGames} />
      {games.map(game => {
        return (<GameSection key={game.uuid} game={game}/>);
      })}

    </Box>
  );
}
