import React from "react";
import { Box, Typography } from "@mui/material";


export default function ResultDisplay({guessRes}) {
  if (guessRes === "AFM") {
    return (
      <Typography>You're on the right track, but your guess was too short!</Typography>
    );
  }

  if (guessRes.Correct) {
    const {user_guess, answer, points_earned, new_lifeline} = guessRes.Correct;
    return (
      <Box>
        <Typography>Correct! You earned {points_earned} points</Typography>
        {new_lifeline && <Typography>You also got a {new_lifeline} lifeline!</Typography>}
        <Typography>Your Guess:</Typography>
        <FlaggedText text={user_guess.text} flags={user_guess.flags}/>
        <Typography>Correct Answer:</Typography>
        <FlaggedText text={answer.text} flags={answer.flags}/>
      </Box>
    )
  }

  if (guessRes.Incorrect) {
    const {user_guess, answer} = guessRes.Incorrect;
    return (
      <Box>
        <Typography>Incorrect! The Game is now over. Better luck next time!</Typography>
        <Typography>Your Guess:</Typography>
        <FlaggedText text={user_guess.text} flags={user_guess.flags}/>
        <Typography>Correct Answer:</Typography>
        <FlaggedText text={answer.text} flags={answer.flags}/>
      </Box>
    )
  }


  return (
    <Typography>Error: Received a guess result in an unexpected format: {JSON.stringify(guessRes)}</Typography>
  );
}

function FlaggedText({text, flags}) {
  return <Typography>{text}</Typography>
}
