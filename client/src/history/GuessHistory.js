import axios from "axios";
import React from "react";
import { Typography, Box, CircularProgress, Link, Divider } from "@mui/material";
import {
  ALBUM_LOGOS, generateGameHref, generateSongHref,
  unescapeQuestionMarks, useSearchParamsState
} from "../utils/Utils";
import { generateFlags } from "./GameDetails";
import { FlaggedText } from "../game/ResultDisplay";
import { parseISO } from "date-fns";

export default function GuessHistory() {
  const [data, setData] = React.useState({});
  const [fetchError, setFetchError] = React.useState(false);
  const album = useSearchParamsState("album", "")[0];
  const song = useSearchParamsState("song", "")[0];
  const prompt = useSearchParamsState("prompt", "")[0];

  React.useEffect(() => {
    axios.get(`/history/line?album=${album}&song=${song}&prompt=${prompt}`).then((response) => {
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
      <>
        <div>
          There was an error fetching the content! Please check the values in the url.
          If you think this is a bug, please let me know.
        </div>
        <Box>
          <Typography>
            album: {album}
          </Typography>
          <Typography>
            song: {song}
          </Typography>
          <Typography>
            prompt: {prompt}
          </Typography>
        </Box>
      </>
    );
  } else if (JSON.stringify(data) == "{}") {
    return (
      <CircularProgress />
    );
  }


  return (
    <Box m={2} display="flex" flexDirection="column" gap={1} >
      <Box sx={{border: "3px solid #B9D9EB", borderRadius: "5px"}} p={1}>
        <Box display="flex" alignItems="center" justifyContent="center">
          <Box
            component="img"
            sx={{
              height: "3em",
              width: "3em",
            }}
            alt="Album Img"
            src={ALBUM_LOGOS[album]}
            mr={1}
          />
          <Typography variant="h6">
            <Link href={generateSongHref(album, song)}>
              {unescapeQuestionMarks(album)} : {unescapeQuestionMarks(song)}
            </Link>
          </Typography>
        </Box>
        {data.length == 1 ?
          (
            <Typography variant="body1">
              There has been {data.length} guess for the line:
            </Typography>
          ) : (
            <Typography variant="body1">
              There have been {data.length} guesses for the line:
            </Typography>
          )
        }
        <Typography variant="h4">
          <span style={{color:"#00ab66", fontFamily: "Monospace", fontWeight: "bold"}}>
            {unescapeQuestionMarks(prompt)}
          </span>
        </Typography>
      </Box>

      <Box display="flex" flexDirection="column" gap={1}>
        {data.map((object, index) => (<GuessDetails guess={object} key={index}></GuessDetails>))}
      </Box>
    </Box>
  );
}

function GuessDetails({ guess }) {
  const {correct_answer, user_guess, points_earned, lifelines_used, player_name} = guess;
  const was_multiple_choice = guess.options.length > 0;

  let {guessFlags, answerFlags} = generateFlags(user_guess, correct_answer);

  if (lifelines_used.includes("Skip") || (was_multiple_choice && guess.result === "correct")) {
    answerFlags = answerFlags.map(() => -1);
    guessFlags = guessFlags.map(() => -1);
  }
  if (was_multiple_choice && guess.result === "incorrect") {
    answerFlags = answerFlags.map(() => 1);
    guessFlags = guessFlags.map(() => 1);
  }

  const options = { year: "numeric", month: "numeric", day: "numeric" };
  const submit_time = parseISO(guess.submit_time).toLocaleDateString(undefined, options);

  return (
    <Box sx={{border: "3px solid #a3c1ad", borderRadius: "5px"}} p={1} width="100%">
      <Box display="flex" flexDirection="column">
        <Box display="flex" alignItems="center">
          <Typography>
            Guessed by {player_name ?
              (<span style={{color:"#00ab66"}}>{player_name}</span>):
              (<span style={{color:"darkgray", fontWeight: "bold"}}>{"<Anonymous>"}</span>)
            } on {}
            <Link href={generateGameHref(guess.game_uuid)}>
              {submit_time}
            </Link>
          </Typography>
        </Box>

        <Box display="flex" flexDirection="column">
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
          {} point{points_earned === 1 ? "" : "s"} earned
        </Typography>
      </Box>
    </Box>
  );
}

const bg = "#d8ecf3";

export function LinePopoverContent({ album, song, prompt }) {
  const [data, setData] = React.useState({});
  const [fetchError, setFetchError] = React.useState(false);

  React.useEffect(() => {
    axios.get(`/history/line?album=${album}&song=${song}&prompt=${prompt}`).then((response) => {
      setData(response.data);
      if (response.status !== 200) {
        setFetchError(true);
      }
    }).catch(function() {
      setFetchError(true);
    });
  }, [album, song, prompt]);

  if (fetchError) {
    return (
      <Box sx={{background:bg}}>
        <div>
          Error
        </div>
      </Box>
    );
  } else if (JSON.stringify(data) == "{}") {
    return (
      <Box sx={{background:bg}} overflow="hidden">
        <CircularProgress />
      </Box>
    );
  }


  return (
    <Box sx={{background:bg}} p={1}>
      {data.length == 0 && <Typography> No guesses yet!</Typography>}
      {data.map((guess, index) => {
        let {user_guess, correct_answer, lifelines_used} = guess;
        const was_multiple_choice = guess.options.length > 0;


        let {guessFlags} = generateFlags(user_guess, correct_answer);

        if (
          lifelines_used.includes("Skip") || (was_multiple_choice && guess.result === "correct")
        ) {
          guessFlags = guessFlags.map(() => -1);
        }
        if (was_multiple_choice && guess.result === "incorrect") {
          guessFlags = guessFlags.map(() => 1);
        }

        if (lifelines_used.includes("Skip")) {
          user_guess = "<Skipped>";
        }


        return (
          <FlaggedText
            key={index}
            text={user_guess}
            flags={guessFlags}
          />
        );
      })}
    </Box>
  );
}