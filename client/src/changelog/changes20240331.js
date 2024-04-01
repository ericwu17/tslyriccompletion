import React from "react";
import {
  Box,
  Link,
  Typography,
} from "@mui/material";


export default function Changes20240331() {
  return (
    <Box m={2}>
      <Typography variant="h4">
        Changes made March 31, 2024
      </Typography>

      <Box m={2}></Box>

      <Typography variant="h5">
        Reorganized the lyrics
      </Typography>
      <Typography>
        For better consistency, I have changed all {}
        <tt>
          -in'
        </tt> endings to {}
        <tt>
          -ing
        </tt>.
        So <tt>gettin'</tt> becomes <tt>getting</tt>, {}
        <tt>standin'</tt> becomes <tt>standing</tt>, etc.
        In general, you can now expect the <tt>-ing</tt> ending to be used.
      </Typography>

      <Typography>
        Some lines which were too short were combined into longer lines.
      </Typography>

      <Typography>
        Many background vocal lines were deleted.
      </Typography>

      <Typography>
        The mattress line in Better than revenge was finally changed to be about matches.
      </Typography>

      <Box m={2}></Box>
      <Typography variant="h5">
        Changes to the game's logic
      </Typography>
      <Typography>
        Previously, if a line had multiple successors, then it would have never been shown as a
        prompt. However, this seems a little bit too restrictive. Now, such a line may still be used
        as a prompt, and based on whatever the player has guessed, the game server shall determine
        what the "actual" correct answer is, by picking whatever next line would have given the
        player
        a higher score. In other words, the game will try to figure out the position in the song
        where the player is completing the lyric.
      </Typography>
      <Typography>
        When a song has multiple successors, a multiple choice question still has only one
        correct choice out of 17. The correct line is randomly chosen.
      </Typography>
      <Typography>
        The criteria for the "on the right track" has finally been fixed.
        You should notice that you will receive this message much more rarely,
        and you will actually be on the right track when you see it.
      </Typography>

      <Box m={2}></Box>
      <Typography variant="h5">
        Feedback form
      </Typography>
      <Typography>
        Please let me know what you think of these changes by using
        the <Link href="/feedback">feedback form</Link>!
      </Typography>
    </Box>
  );
}