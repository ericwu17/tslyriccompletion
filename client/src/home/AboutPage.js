import React from "react";
import { Box, Typography, Link } from "@mui/material";

export default function AboutPage() {
  return <Box mt={2} mx={5} mb={30}>
    <Typography variant="h3" sx={{textDecoration: 'underline', mb: 2}}>
      Taylor Swift Lyric Guessing Game
    </Typography>
    <Typography>
      This is a game where your goal is to guess the next line from a random Taylor Swift song! To start playing, press <Link href="/play">start game</Link> in the navbar! You can also use this site to browse Taylor Swift lyrics, although you might
      find the line highlighting to be a little distracting.
    </Typography>
    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      How are lyrics selected?
    </Typography>
    <Typography>
      Firstly, the game selects a random song. It then attempts to pick a random lyric from the song, while trying to avoid some bad cases. This means that each song is equally likely
      to appear, even though some songs might have many more lines. 
    </Typography>
    <Typography>
      As an example of a "bad case", in the song <Link href="/song/Speak Now/Long Live">Long Live</Link>,
      the line "Long live the walls we crashed through" can be followed by either "I had the time of my life, with you" or "All the kingdom lights shined just for me and you", depending on the
      location within the song. Therefore, the game will not show the line "Long live the walls we crashed through" and ask you to guess what comes next.
    </Typography>
    <Typography>
      There are also some lines which I call "exclamatory", such as {' '}
        <Link href="/song/Reputation/Dress">Ah, ha, ha, ha-ah</Link>, {' '}
        <Link href="/song/Speak Now/Haunted">Oh, oh, oh, oh, oh, oh, oh, oh, oh</Link>, {' '}
        <Link href="/song/Fearless/Fifteen">La-la-la, la-la-la, la-la-la-la</Link>.
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

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Known issues
    </Typography>
    <Typography>
      When viewing the track list of Red, I removed the song "All Too Well" (non-ten minute version), since it contains duplicate lyrics with the ten minute version. This results in a mismatch between track numbers and songs. (For example, Come Back Be Here is track 18 on Red (TV), but it appears as track 17 on this site).
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Future Ideas for the Taylor Swift Guessing Game?
    </Typography>
    <Typography>
      I'll write any future ideas here! Please let me know if you have any suggestions.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Acknowledgements
    </Typography>
    <Typography>
      Big thanks to Jake Thompson for <Link href="https://github.com/wjakethompson/taylor">compiling Taylor Swift lyrics</Link>! This game would not have been possible without these.
    </Typography>
    <Typography>
      Thanks to my friends Kim and Hannah for being my most frequent testers of early versions of this lyric guessing game.
    </Typography>
    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Source Code
    </Typography>
    <Typography>
      The source code for this project is available at <Link href="https://github.com/EricWu2003/taylorlyricguessingrs">https://github.com/EricWu2003/taylorlyricguessingrs</Link>.
    </Typography>
    <Typography>
      My name is Eric, and I'm an undergraduate student at UCLA (expected graduation in spring of 2025). I love listening to Taylor Swift (surprise, right?) and if you have issues with trying to run the code
      locally, please reach out! I'd love to hear from you. My email is eric.dianhao.wu@gmail.com. Also feel free to reach out if you have any feedback about the game, and definitely let me know
      if you encounter anything you think may be a bug!
    </Typography>
  </Box>;
}