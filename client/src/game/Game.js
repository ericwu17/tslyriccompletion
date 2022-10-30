import React from "react";
import axios from "axios";
import { Button } from "@mui/material";
import GameStateDisplay from "./GameStateDisplay";


export default function Game() {
  const [hasStarted, setHasStarted] = React.useState(false);
  const [gameState, setGameState] = React.useState({});


  const beginGame = () => {
    axios.post(`/game/start`, [["Speak Now", "Speak Now"]]).then((response) => {
      setGameState(response.data);
      setHasStarted(true);
    })
    setHasStarted(true);
  }

  if (!hasStarted) {
    return (
      <Button onClick={beginGame}>
        Begin
      </Button>
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
