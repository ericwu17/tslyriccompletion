import axios from "axios";
import React from "react";
import { ALBUM_LOGOS, generateSongHref, useSearchParamsState } from "../utils/Utils";
import { differenceInSeconds, parseISO } from "date-fns";
import {
  Box, Table, TableRow, TableCell, Divider,
  TableBody, CircularProgress, Typography, Link
} from "@mui/material";
import InclusionExclusionSection from "./InclusionExclusionSection";
import { FlaggedText } from "../game/ResultDisplay";
import { diffChars } from "diff";
import levenshtein from "js-levenshtein";

export default function GameDetails() {
  const [data, setData] = React.useState({});
  const [fetchError, setFetchError] = React.useState(false);
  const id = useSearchParamsState("id", "")[0];

  React.useEffect(() => {
    axios.get(`/history/game?id=${id}`).then((response) => {
      setData(response.data);
      if (response.status !== 200) {
        setFetchError(true);
      }
    }).catch(function() {
      setFetchError(true);
    });
  }, []);

  if (fetchError) {
    return (
      <div>
        There was an error fetching the content! Please check the "id" value in the url.
        If you think this is a bug, please let me (Eric) know.
      </div>
    );
  } else if (JSON.stringify(data) == "{}") {
    return (
      <CircularProgress />
    );
  }

  let {game, guesses} = data;

  const name = game.player_name || "<Anonymous>";

  const score = game.terminal_score;

  const startTime = parseISO(game.start_time);
  let numGuesses = guesses.length;

  let previous_time = parseISO(game.start_time);
  for (let guess of guesses) {
    let time_now = parseISO(guess.submit_time);
    guess.time_elapsed = differenceInSeconds(time_now, previous_time);
    previous_time = time_now;
  }

  return (
    <Box m={2} display="flex" flexDirection="column" alignItems="center" gap={1} >
      <Box sx={{border: "3px solid #B9D9EB", borderRadius: "5px"}} p={1}>
        <Table>
          <TableBody>
            <TableRow>
              <TableCell><strong>Start time: {}</strong></TableCell>
              <TableCell>{startTime.toLocaleString()}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Played By: {}</strong></TableCell>
              <TableCell>{name}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Final Score: {}</strong></TableCell>
              <TableCell>{score}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell><strong>Number of Guesses: {}</strong></TableCell>
              <TableCell>{numGuesses}</TableCell>
            </TableRow>
            <TableRow>
              <TableCell colSpan={2}>
                <InclusionExclusionSection selectedSongs={game.selected_songs}/>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </Box>

      <Box display="flex" flexDirection="column" gap={1}>
        {guesses.map(guess => {
          return (
            <GuessDetails
              key={guess.order_num}
              guess={guess}
              totalNumGuesses={numGuesses}
            />
          );
        })}
      </Box>

    </Box>
  );
}


function GuessDetails({ guess, totalNumGuesses }) {
  const {correct_answer, user_guess, prompt, points_earned, time_elapsed, lifelines_used} = guess;
  const was_multiple_choice = guess.options.length > 0;

  const href = generateSongHref(guess.album, guess.song_name);

  let {guessFlags, answerFlags} = generateFlags(user_guess, correct_answer);

  if (lifelines_used.includes("Skip") || (was_multiple_choice && guess.result === "correct")) {
    answerFlags = answerFlags.map(() => -1);
    guessFlags = guessFlags.map(() => -1);
  }
  if (was_multiple_choice && guess.result === "incorrect") {
    answerFlags = answerFlags.map(() => 1);
    guessFlags = guessFlags.map(() => 1);
  }

  return (
    <Box sx={{border: "3px solid #a3c1ad", borderRadius: "5px"}} p={1} width="100%">
      <Box display="flex" flexDirection="column">
        <Box display="flex" alignItems="center">
          <Box
            component="img"
            sx={{
              height: "1em",
              width: "1em",
            }}
            alt="Album Img"
            src={ALBUM_LOGOS[guess.album]}
            mr={1}
          />
          <Link href={href} target="_blank" mr={1}>
            {guess.album} : {guess.song_name}
          </Link>
          {} (Question {guess.order_num+1} of {totalNumGuesses})
        </Box>

        <Box display="flex" flexDirection="column">
          <Box display="flex">
            <Box mr={1} alignItems="flex-end">
              <Typography sx={{ fontFamily: "Monospace", color: "gray", fontWeight: "bold" }}>
                Prompt:
              </Typography>
            </Box>
            <Box>
              <FlaggedText text={prompt} flags={3}/>
            </Box>
          </Box>
          <Box display="flex">
            <Box mr={1} alignItems="flex-end">
              <Typography sx={{ fontFamily: "Monospace", color: "gray", fontWeight: "bold" }}>
                |Guess:
              </Typography>
            </Box>
            <Box>
              <FlaggedText text={user_guess} flags={guessFlags}/>
            </Box>
          </Box>
          <Box display="flex">
            <Box mr={1} alignItems="flex-end">
              <Typography sx={{ fontFamily: "Monospace", color: "gray", fontWeight: "bold" }}>
                Actual:
              </Typography>
            </Box>
            <Box>
              <FlaggedText text={correct_answer} flags={answerFlags}/>
            </Box>
          </Box>
        </Box>
      </Box>
      <Divider />

      {was_multiple_choice && (
        <Typography>
          This question was multiple choice.
        </Typography>
      )}
      {lifelines_used.length > 0 && (
        <Typography>
          Used the {}
          {lifelines_used.join(", ")}
          {} lifeline{lifelines_used.length === 1 ? "" : "s"}.
        </Typography>
      )}


      <Divider />
      <Box>
        <Typography>
          <span style={{color: "#508124", fontWeight: "bold"}}>
            {points_earned}
          </span>
          {} point{points_earned === 1 ? "" : "s"} earned, {time_elapsed} seconds elapsed
        </Typography>
      </Box>
    </Box>
  );
}

function generateFlags(guess, answer) {

  // First we need to figure out the optimal amount to truncate the user guess
  // (This is the truncation amount that minimizes distance)
  let optimalTruncateAmt = guess.length;
  let minDist = Infinity;
  for (let i = guess.length; i >= 0; i --) {
    let d = levenshtein(guess.slice(0, i), answer);
    if (d < minDist) {
      minDist = d;
      optimalTruncateAmt = i;
    }
  }

  // initialize the flags, and set the truncated characters to a yellow
  // color (flag code = 2)
  // Note that it is fine if we leave the unaffected indices as "null" instead of setting them to
  // the integer 0, since our component FlaggedText assumes that all unrecognized flags are 0.
  let guessFlags =  guess.split("").map(() => null);
  let answerFlags =  answer.split("").map(() => null);

  for (let i = guess.length; i > optimalTruncateAmt; i --) {
    guessFlags[i-1] = 2;
  }

  // truncate the guess
  guess = guess.slice(0, optimalTruncateAmt);

  // generate diff objects
  const diffObjects = diffChars(guess, answer, {ignoreCase: true});

  // expand the diff objects into an array of integers.
  let guessIndex = 0;
  let answerIndex = 0;
  for (let d of diffObjects) {
    if (!d.added && !d.removed) {
      guessIndex += d.count;
      answerIndex += d.count;
    } else if (d.added) {
      for (let i = 0; i < d.count; i ++) {
        answerFlags[answerIndex] = 1;
        answerIndex += 1;
      }
    }  else if (d.removed) {
      for (let i = 0; i < d.count; i ++) {
        guessFlags[guessIndex] = 1;
        guessIndex += 1;
      }
    }
  }

  return {
    guessFlags,
    answerFlags,
  };
}
