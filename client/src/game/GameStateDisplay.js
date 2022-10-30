import React from "react";
import axios from "axios";
import { Box, Button, TextField, Typography } from "@mui/material";
import ResultDisplay from "./ResultDisplay";


export default function GameStateDisplay({gameState, setGameState, setHasStarted, restartGame}) {
  const [guessResult, setGuessResult] = React.useState({});
  const [currentGuess, setCurrentGuess] = React.useState("");
  // console.log(currentGuess);


  // console.log(gameState)

  const { score, current_question, id, completed_question, terminated } = gameState;
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
        if (guess_res !== "AFM") {
          setCurrentGuess("");
        }
      })
    } else if (e.key === "Enter" && completed_question && !terminated) {
      goToNextQuestion();
    }
  }

  const goToNextQuestion = () => {
    axios.get(`/game/next?id=${id}`).then((response) => {
      setGameState(response.data);
      setGuessResult({});
    })
  }

  const beginAgain = () => {
    restartGame()
    setGuessResult({});
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
      {Object.keys(guessResult).length > 0 && <ResultDisplay guessRes={guessResult}/>}

      {completed_question && !terminated && <Button onClick={goToNextQuestion}>Next Question</Button>}
      {completed_question && terminated && <Button onClick={beginAgain}>Play Again</Button>}
    </>
  )
}
