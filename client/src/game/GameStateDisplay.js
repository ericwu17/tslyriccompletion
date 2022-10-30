import React from "react";
import axios from "axios";
import { Box, Button, Link, TextField, Typography } from "@mui/material";
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
      {!completed_question && 
        <LifelineSection 
          gameState={gameState}
          setGameState={setGameState}
        />
      }

      {current_question.answer && <DisplayAnswer question={current_question}/>}

      {completed_question && !terminated && <Button onClick={goToNextQuestion}>Next Question</Button>}
      {completed_question && terminated && <Button onClick={beginAgain}>Play Again</Button>}
    </>
  )
}

function LifelineSection({gameState, setGameState}) {
  const {id, hints_shown} = gameState
  const {show_title_album, show_prev_lines, skip} = gameState.lifeline_inv;

  const consumeLifeline = lifelineToUse => {
    axios.get(`/game/use-lifeline?id=${id}&lifeline=${lifelineToUse}`).then((response) => {
      setGameState(response.data);
    })
  }

  let nameHint = hints_shown.find(hint => hint.ShowTitle);
  nameHint = nameHint && nameHint.ShowTitle;
  let prevLineHint = hints_shown.find(hint => hint.ShowPrevLines);
  prevLineHint = prevLineHint && prevLineHint.ShowPrevLines;


  return (
    <Box display="flex" flexDirection="column" alignItems="center">
      {!nameHint && <Button onClick={() => {consumeLifeline("show_title_album")}} disabled={show_title_album === 0}>
        Show song album and name ({show_title_album})
      </Button>}
      {nameHint && <Typography>{nameHint}</Typography>}
      
      {!prevLineHint && <Button onClick={() => {consumeLifeline("show_prev_lines")}} disabled={show_prev_lines === 0}>
        Show previous lines ({show_prev_lines})
      </Button>}
      {prevLineHint && 
        <>
          {prevLineHint.is_at_song_beginning && <Typography sx={{color:'red'}}>This is the beginning of the song:</Typography>}
          {!prevLineHint.is_at_song_beginning && <Typography>...</Typography>}
          <Typography style={{whiteSpace: 'pre-line'}}>{prevLineHint.lines}</Typography>
        </>
      }

      <Button onClick={() => {consumeLifeline("skip")}} disabled={skip === 0}>
        Skip Question({skip})
      </Button>
    </Box>
  )

}

function DisplayAnswer({question}) {
  const [showLyrics, setShowLyrics] = React.useState(false);

  const {song, shown_line} = question;
  const albumTitle = `${song.album}--${song.name}`;
  const lines = song.lines;

  const toggleShowLyrics = () => {
    setShowLyrics(!showLyrics);
  }

  return (
    <Box display="flex" flexDirection="column" alignItems="center">
      <Typography>
        This question was from: {albumTitle} <Link onClick={toggleShowLyrics}>({showLyrics ? "hide" :"show"})</Link>
      </Typography>
      {showLyrics && lines.map((line, index) => {
        const text = line.text
        return <Typography key={index} sx={{color: text === shown_line ? 'red' : 'black'}}>{text}</Typography>
      })}

    </Box>
  )

}
