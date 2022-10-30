import React from "react";
import axios from "axios";
import { Box, Button, TextField, Typography } from "@mui/material";


export default function GameStateDisplay({gameState, setGameState, setHasStarted}) {
  const [guessResult, setGuessResult] = React.useState({});
  const [currentGuess, setCurrentGuess] = React.useState("");
  // console.log(currentGuess);


  // console.log(gameState)

  const { score, current_question, id, completed_question } = gameState;
  console.log(`The current game has id: ${id}`)

  if (!current_question) {
    return <Typography>
      There was an issue fetching the game :(
    </Typography>
  }

  const prompt = current_question.shown_line;


  const onKeyDown = e => {
    if (e.key === "Enter" && !completed_question && currentGuess !== "") {
      // submit the guess
      axios.get(`/game/submit-guess?id=${id}&guess=${currentGuess}`).then((response) => {
        const {game_state, guess_res} = response.data;
        console.log(response.data);
        setGameState(game_state);
        setGuessResult(guess_res);
        setCurrentGuess("");
      })
    }
  }

  const goToNextQuestion = () => {
    axios.get(`/game/next?id=${id}`).then((response) => {
      setGameState(response.data);
    })
  }

  return (
    <>
      <Typography>
        Current Score: {score}
      </Typography>
      <Typography>
        What line follows: {prompt}
      </Typography>
      <Box display="flex" flexDirection="row">
        <TextField 
          onChange={event => setCurrentGuess(event.target.value)}
          onKeyDown={onKeyDown}
          value={currentGuess}
          sx={{width: '100%'}}
        />
      </Box>

      {completed_question && <Button onClick={goToNextQuestion}>Next Question</Button>}
    </>
  )
  
}
