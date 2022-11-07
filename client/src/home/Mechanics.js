import React from "react";
import { Box, Typography, Link } from "@mui/material";

export default function MechanicsPage() {
  return <Box mt={2} mx={5} mb={30}>
    <Typography variant="h3" sx={{textDecoration: 'underline', mb: 2}}>
      Game Mechanics
    </Typography>
   <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      How are lyrics selected?
    </Typography>
    <Typography>
      Firstly, the game selects a random song. It then attempts to pick a random lyric from the song, while trying to avoid some bad cases. This means that each song is equally likely
      to appear, even though some songs might have many more lines. 
    </Typography>
    <Typography>
      As an example of a "bad case", in the song <Link href="/tswift/song/Speak Now/Long Live">Long Live</Link>,
      the line "Long live the walls we crashed through" can be followed by either "I had the time of my life, with you" or "All the kingdom lights shined just for me and you", depending on the
      location within the song. Therefore, the game will not show the line "Long live the walls we crashed through" and ask you to guess what comes next.
    </Typography>
    <Typography>
      There are also some lines which I call "exclamatory", such as {' '}
        <Link href="/tswift/song/Reputation/Dress">Ah, ha, ha, ha-ah</Link>, {' '}
        <Link href="/tswift/song/Speak Now/Haunted">Oh, oh, oh, oh, oh, oh, oh, oh, oh</Link>, {' '}
        <Link href="/tswift/song/Fearless/Fifteen">La-la-la, la-la-la, la-la-la-la</Link>.
      Can you figure out which songs these lines come from? I certainly can't, and I don't think it's fun to include such lines in the guessing game. In this game, neither the prompt nor the answer will
      ever be an exclamatory line. An exclamatory line is any line with more than a 50% concentration of exclamatory words, or any line with less than 3 words 
      (short lines are excluded because they are generally hard to identify). A list of exclamatory words is:
      <Typography sx={{ml:5}}>
        "mmmm", "mmm", "mm", "oh", "ohh", "ooh", "la", "na", "no", "my", "uh", "huh", "ahh", "ah", "ha", "yeah", "whoa",
        "ayy", "i", "eh", "hey", "ra", "di", "da"
      </Typography>
    </Typography>
    <Typography>
      When browsing lyrics on this site, all lines which may show up as a prompt in-game are shown in black, while all lines which are not candidates for in-game prompts are shown in pink. 
      Hovering over a pink line shows reasons why it is not a valid candidate.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      What are lifelines?
    </Typography>
    <Typography>
      TODO: This section of the about page is coming soon.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      How is my score calculated?
    </Typography>
    <Typography>
      TODO: This section of the about page is coming soon.
    </Typography>
  </Box>;
}