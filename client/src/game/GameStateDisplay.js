import React from "react";
import axios from "axios";
import Cookies from "js-cookie";
import { Box, Button, Divider, Link, TextField, Typography } from "@mui/material";
import ResultDisplay from "./ResultDisplay";
import { ALBUM_LOGOS, generateSongHref, normalizeQuotes } from "../utils/Utils";

const MAX_NAME_LEN = 35;

export default function GameStateDisplay({gameState, setGameState, setHasStarted}) {
  const [guessResult, setGuessResult] = React.useState({});
  const [currentGuess, setCurrentGuess] = React.useState("");
  const [currentName, setCurrentName] = React.useState("");


  const { score, current_question, id, completed_question, terminated, choices } = gameState;

  React.useEffect(() => {
    if (terminated === true) {
      // once the game ends, we want to remove the cookie so that
      // if users close the tab, they won't be loaded into the game
      // next time
      Cookies.remove("tsgg-game-id");
    }
  }, [terminated]);

  React.useEffect(() => {

    if (currentName.length > MAX_NAME_LEN) {
      setCurrentName(currentName.slice(0, MAX_NAME_LEN));
    }
  }, [currentName]);

  if (!current_question) {
    return (
      <Typography>
        There was an issue fetching the game :(
      </Typography>
    );
  }

  const isMultipleChoice = choices.length > 0;

  const prompt = current_question.shown_line;




  const onKeyDown = e => {
    if (e.key === "Enter" && !completed_question && currentGuess !== "") {
      submitGuess(normalizeQuotes(currentGuess));
    } else if (e.key === "Enter" && completed_question && !terminated) {
      goToNextQuestion();
    }
  };

  const submitGuess = guess => {
    if (gameState.completed_question) {
      return;
    }

    axios.get(`/game/submit-guess?id=${id}&guess=${guess}`).then((response) => {
      const {game_state, guess_res} = response.data;
      setGameState(game_state);
      setGuessResult(guess_res);
      if (!guess_res.AFM) {
        setCurrentGuess("");
      }
    });
  };

  const goToNextQuestion = () => {
    axios.get(`/game/next?id=${id}`).then((response) => {
      setGameState(response.data);
      setGuessResult({});
    });
  };

  const handleMultipleChoiceClick = () => {
    if (!completed_question) {
      axios.get(`/game/reduce-multiple-choice?id=${id}`).then((response) => {
        setGameState(response.data);
      });
    }
  };

  const beginAgain = () => {
    if (currentName !== "") {
      claimGame(currentName);
    }
    setGuessResult({});
    setHasStarted(false);
  };

  const claimboxOnKeyDown = e => {
    if (e.key === "Enter" && currentName !== "") {
      claimGame(currentName);
    }
  };

  const claimGame = name => {
    setCurrentName("");
    axios.get(`/game/claim?id=${id}&name=${name}`).then((response) => {
      if (response.status === 200) {
        setGuessResult({});
        setHasStarted(false);
      }
    });
  };

  return (
    <Box m={2}>
      <Typography>
        Current Score: {score}
      </Typography>
      <Typography>
        What line follows: {prompt}
      </Typography>
      {!isMultipleChoice &&
        <Box display="flex" flexDirection="row">
          <TextField
            onChange={event => setCurrentGuess(event.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Enter your guess here..."
            value={currentGuess}
            sx={{width: "100%"}}
          />
          <Button onClick={handleMultipleChoiceClick}>
            Multiple Choice
          </Button>
        </Box>
      }
      {isMultipleChoice &&
        <Box display="flex" flexDirection="column">
          <Typography>Your {choices.length} choices are: </Typography>
          {choices.map((choice, index) => {
            if (!completed_question) {
              return (
                <Typography key={index}>
                  {index+1}) {}
                  <Link onClick={() => submitGuess(choice)}>{choice}</Link>
                </Typography>
              );
            } else {
              return (
                <Typography key={index} >
                  {index+1}) {}
                  <span style={{color:"darkgray", fontWeight: "bold"}}>{choice}</span>
                </Typography>
              );
            }
          })
          }
        </Box>
      }


      {Object.keys(guessResult).length > 0 && <ResultDisplay guessRes={guessResult}/>}
      {!completed_question &&
        <LifelineSection
          gameState={gameState}
          setGameState={setGameState}
          setGuessResult={setGuessResult}
        />
      }

      {completed_question && !terminated &&
        <Button onClick={goToNextQuestion} size="large">Next Question</Button>
      }
      {completed_question && terminated &&
        <Box>
          <Typography sx={{color:"#BA0021"}}>
            Good game! Better luck next time!
            Leave your name if you want to be remembered ({MAX_NAME_LEN} characters max):
          </Typography>
          <Box>
            <TextField
              placeholder="Enter your name..."
              onChange={event => setCurrentName(event.target.value)}
              onKeyDown={claimboxOnKeyDown}
              value={currentName}
            />
            <Button onClick={beginAgain} sx={{width:"min-content"}}>Play Again</Button>
          </Box>
        </Box>
      }

      {current_question.answer && (
        <>
          <Box mb={1}/>
          <Divider />
          <Divider />
          <Box mb={2}/>
          <DisplayAnswer question={current_question}/>
        </>
      )}
    </Box>
  );
}

function LifelineSection({gameState, setGameState, setGuessResult}) {
  const {id, hints_shown} = gameState;
  const {show_title_album, show_prev_lines, skip} = gameState.lifeline_inv;

  const consumeLifeline = lifelineToUse => {
    axios.get(`/game/use-lifeline?id=${id}&lifeline=${lifelineToUse}`).then((response) => {
      const newGameState = response.data;
      setGameState(newGameState);

      if (lifelineToUse === "skip") {
        setGuessResult({
          Skipped: {
            user_guess: {
              text: "-",
              flags: [],
            },
            answer: {
              text: newGameState.current_question.answer,
              flags: [],
            },
          }
        });
      }
    });
  };

  let nameHint = hints_shown.find(hint => hint.ShowTitle);
  nameHint = nameHint && nameHint.ShowTitle;
  let prevLineHint = hints_shown.find(hint => hint.ShowPrevLines);
  prevLineHint = prevLineHint && prevLineHint.ShowPrevLines;


  return (
    <Box display="flex" flexDirection="column" alignItems="center">
      {!nameHint &&
        <Button
          onClick={() => {consumeLifeline("show_title_album");}}
          disabled={show_title_album === 0}
        >
          Show song album and name ({show_title_album} left)
        </Button>
      }
      {nameHint &&
        <Box display="flex" flexDirection="row" alignItems="center">
          <Box
            component="img"
            sx={{
              height: 20,
              width: 20,
            }}
            alt="Album Img"
            src={ALBUM_LOGOS[nameHint.split(" : ")[0]]}
            mx={0.5}
          />
          <Typography>{nameHint}</Typography>
        </Box>
      }

      {!prevLineHint &&
        <Button
          onClick={() => {consumeLifeline("show_prev_lines");}}
          disabled={show_prev_lines === 0}
        >
          Show previous lines ({show_prev_lines} left)
        </Button>
      }
      {prevLineHint &&
        <>
          {prevLineHint.is_at_song_beginning &&
            <Typography sx={{color:"red"}}>This is the beginning of the song:</Typography>
          }
          {!prevLineHint.is_at_song_beginning && <Typography>...</Typography>}
          <Typography style={{whiteSpace: "pre-line"}}>{prevLineHint.lines}</Typography>
        </>
      }

      <Button onClick={() => {consumeLifeline("skip");}} disabled={skip === 0}>
        Skip Question ({skip} left)
      </Button>
    </Box>
  );

}

function DisplayAnswer({question}) {
  const [showLyrics, setShowLyrics] = React.useState(true);

  const {song, shown_line} = question;
  const albumTitle = `${song.album}--${song.name}`;
  const href = generateSongHref(song.album, song.name);
  const { lyrics_raw } = song;


  const toggleShowLyrics = () => {
    setShowLyrics(!showLyrics);
  };

  return (
    <Box display="flex" flexDirection="column" alignItems="center">
      <Box
        display="flex"
        flexDirection="row"
        alignItems="center"
        justifyContent="center"
        width="100%"
      >
        <Typography variant="h4" noWrap>
          <Link href={href} target="_blank">{albumTitle}</Link>
        </Typography>

        <Box
          component="img"
          sx={{
            height: 35,
            width: 35,
          }}
          alt="Album Img"
          src={ALBUM_LOGOS[song.album]}
          mx={1}
        />
        <Button onClick={toggleShowLyrics}>({showLyrics ? "hide" :"show"} lyrics)</Button>
      </Box>
      {showLyrics && <SongLyricsDisplay lyrics_raw={lyrics_raw} shown_line={shown_line}/>}

    </Box>
  );

}

function SongLyricsDisplay({lyrics_raw, shown_line}) {
  const lines = lyrics_raw.split("\n");
  return (
    <Box>
      {lines.map((line, index) => {
        const styles = {};
        if (line === shown_line) {
          styles["color"] = "red";
        }
        if (line.startsWith("[") && line.endsWith("]")) {
          styles["fontWeight"] = "bold";
        }
        return <Typography key={index} sx={styles}>{line}</Typography>;
      })}
    </Box>
  );
}
