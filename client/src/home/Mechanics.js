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
        the player knows the answer, they may simply type it in. If they need help in determining
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
        song, while trying to avoid some bad cases. This means that each song is equally likely
        to appear, even though some songs might have many more lines.
      </Typography>
      <Typography>
        As an example of a "bad case", in the song {}
        <Link href="/song/Speak Now/Long Live">Long Live</Link>,
        the line "Long live the walls we crashed through" can be followed by either "I had the time
        of my life, with you" or "All the kingdom lights shined just for me and you", depending on
        the location within the song. Therefore, the game will not show the line "Long live the
        walls we crashed through" and ask you to guess what comes next.
      </Typography>
      <Typography>
        There are also some lines which I call "exclamatory", such as {}
        <Link href="/song/Reputation/Dress">Ah, ha, ha, ha-ah</Link>, {}
        <Link href="/song/Speak Now/Haunted">Oh, oh, oh, oh, oh, oh, oh, oh, oh</Link>, or {}
        <Link href="/song/Fearless/Fifteen">La-la-la, la-la-la, la-la-la-la</Link>.
        Can you figure out which songs these lines come from? I certainly can't, and I don't think
        it's fun to include such lines in the guessing game. In this game, neither the prompt nor
        the answer will ever be an exclamatory line. An exclamatory line is any line with more than
        a 50% concentration of exclamatory words, or any line with less than 3 words (short lines
        are excluded because they are generally hard to identify). A list of exclamatory words is:
        <Typography sx={{ml:5}}>
          "mmmm", "mmm", "mm", "oh", "ohh", "ooh", "la", "na", "no", "my", "uh", "huh",
          "ahh", "ah", "ha", "yeah", "whoa", "ayy", "i", "eh", "hey", "ra", "di", "da"
        </Typography>
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
        same character. The game will also check if truncating the player's guess by any amount
        would give a smaller edit distance, and choose to truncate the guess by the best possible
        amount. The motivation here is that if a player thinks the next line is "You're not sorry,
        No, no, oh-oh" and the next line was actually "You're not sorry" then they should still
        receive full credit.
      </Typography>
      <Typography>
        Let <i>d</i> be the edit distance between the player's guess (optimally truncated) and the
        actual next line. If <i>d</i> {">"} 13 then the guess is deemed incorrect and the game ends.
        If <i>d</i> = 0 then the player earns 26 points for a perfect match, as well as a new random
        lifeline. For other values of <i>d</i>, The player receives a score of (14 - <i>d</i>) and
        has a probability (1 - <i>d</i>/13) of receiving a new random lifeline.
      </Typography>
      <Typography>
        There is one other possibility: the game might ask the player for a longer guess if the
        player's guess is more than 5 characters shorter than the next line, yet the player's guess
        is "on the right track". A guess is "on the right track" if you can take a prefix of the
        correct answer such that the edit distance between that prefix and the guess is at most 13.
      </Typography>
    </Box>
  );
}