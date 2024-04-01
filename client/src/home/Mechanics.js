import React from "react";
import { List, ListItem, Box, Typography, Link } from "@mui/material";

export default function MechanicsPage() {
  return (
    <Box my={2} mx={5}>
      <Typography variant="h3" sx={{mb: 2}}>
        Taylor Swift Completion Game: Game Mechanics
      </Typography>
      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Overview
      </Typography>
      <Typography>
        The game is played in individual rounds. Each round, a <i>question</i> is selected, which is
        a line from a Taylor Swift song. The answer to this question will be the next line, and if
        the player knows the answer, they may simply type it in.
        Note that there may be multiple correct answers if the prompt has different
        continuations in different parts of the song.
        If they need help in determining
        the answer, they can use three different types of lifelines (which are a scarce resource),
        and/or reduce the question to a multiple choice question. The game continues indefinitely
        until the player guesses incorrectly, at which point the game ends and the final score is
        determined.
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Selecting the line for the question
      </Typography>
      <Typography>
        Firstly, the game selects a random song. It then attempts to pick a random lyric from the
        song, while trying to avoid some bad cases.
      </Typography>
      <Typography>
        Sometimes, the end of a section is not considered valid prompts, because it is
        too difficult to recall which lines follow them, since there is a long instrumental break.
        Also, some lines are too short to be recognizable.
      </Typography>
      <Typography>
        There are also some lines which are "exclamatory", such as {}
        <Link href="/song/reputation/Dress">Ah, ha, ha, ha-ah</Link>, {}
        <Link href="/song/Speak Now/Haunted">Oh, oh, oh, oh, oh, oh, oh, oh, oh</Link>, or {}
        <Link href="/song/Fearless/Fifteen">La-la-la, la-la-la, la-la-la-la</Link>.
        In this game, neither the prompt nor
        the answer will ever be an exclamatory line. I have used my own discretion to determine
        which lines are considered "exclamatory".
      </Typography>
      <Typography>
        When browsing lyrics on this site, all lines which may show up as a prompt in-game are shown
        in black, while all lines which are not candidates for in-game prompts are shown in blue.
        Hovering over a blue line shows reasons why it is not a valid candidate. Hovering over a
        black line shows all previous guesses made in games by players. For each black line, there
        is a subscript indicating the number of times that line was played in a game.
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Lifelines
      </Typography>
      <Typography>
        There are currently three types of lifelines: ShowTitleAlbum, ShowPrevLines, and Skip.
      </Typography>
      <List sx={{ listStyleType: "disc", listStylePosition:"inside" }}>
        <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
          The ShowTitleAlbum lifeline simply shows you the title of the song, as well as its album.
        </ListItem>
        <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
          The ShowPrevLines lifeline will show the two lines which came before the question's line.
          If there are not enough previous lines because the question's line occurs too early in the
          song, the ShowPrevLines lifeline show everything from the song's first line, and also
          indicate that the lines shown are the initial lines of the song.
        </ListItem>
        <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
          The Skip lifeline skips the current question, showing the answer and earning 0 points for
          the player.
        </ListItem>
      </List>


      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Calculating score (and determining correctness)
      </Typography>
      <Typography>
        For questions which have been reduced to multiple choice, a correct answer will always score
        1 point, and any other answer will be deemed incorrect and end the game. A
        lifeline will never be rewarded for correctly answering a multiple choice question.

      </Typography>
      <Typography>
        For free-response questions (which are the default type), the game uses the{" "}
        <Link href="https://en.wikipedia.org/wiki/Edit_distance">Edit Distance</Link> to calculate
        score. This distance is calculated case insensitively, so "a" and "A" are treated as the
        same character.
        Also, the spaces and punctuation are ignored.
        The game will also check if truncating the player's guess by any amount
        would give a smaller edit distance, and choose to truncate the guess by the best possible
        amount. The motivation here is that if a player thinks the next line is "You're not sorry,
        No, no, oh-oh" and the next line was actually "You're not sorry" then they should still
        receive full credit.
        If there are multiple possible continuations, then the game will pick whichever one
        gives the player the highest score.
      </Typography>
      <Typography>
        Occasionally, the game might tell the player that they are "on the right track"
        and ask for a longer guess if the
        player's guess is not close enough to be correct, but the player's guess is very
        close to some prefix of some correct continuation.
      </Typography>
    </Box>
  );
}