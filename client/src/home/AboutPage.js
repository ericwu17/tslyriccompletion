import React from "react";
import { List, Box, Typography, Link, ListItem } from "@mui/material";

export default function AboutPage() {
  return <Box mt={2} mx={5} mb={30}>
    <Typography variant="h3" sx={{textDecoration: 'underline', mb: 2}}>
      Taylor Swift Lyric Guessing Game
    </Typography>
    <Typography>
      This is a game where your goal is to guess the next line from a random Taylor Swift song! To start playing, press <Link href="/tswift/play">start game</Link> in the navbar! You can also use this site to browse Taylor Swift lyrics, although you might
      find the line highlighting to be a little distracting.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Details on Game Mechanics
    </Typography>
    <Typography>
      See the <Link href="/tswift/mechanics">mechanics</Link> page.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2}}>
      Known issues
    </Typography>
    <Box>
      <List sx={{ listStyleType: 'disc', listStylePosition:'inside' }}>
        <ListItem sx={{ display: 'list-item' }}>
          When viewing the track list of Red, I removed the song "All Too Well" (non-ten minute version), since it contains duplicate lyrics with the ten minute version. This results in a mismatch between track numbers and songs. (For example, Come Back Be Here is track 18 on Red (TV), but it appears as track 17 on this site).
          This mismatch also occurs when the songlist is filtered by a search.
        </ListItem>
        <ListItem sx={{ display: 'list-item' }}>
          In the song <Link href="/tswift/song/Fearless/You All Over Me">You All Over Me</Link>, the line "I lived, I learned" should technically have multiple different successors (it can be followed
          by "And found out what it was to turn around" or "had you, got burned"). However, due to how line breaks are positioned, the game thinks that there is only one possible successor. (This is an issue with determining which lines are valid lines as a prompt).
        </ListItem>
      </List>
    </Box>

    <Typography variant="h4" sx={{textDecoration: 'underline', mb: 2}}>
      Future Ideas for the Taylor Swift Guessing Game?
    </Typography>
    <Typography>
      There's gonna be a high scores page where you can look through game history! Besides from viewing high scores, I also plan to allow you to look at
        statistics about each line of particular songs. It would be cool to see how many times a certain line was played, and to see which lines are commonly
        answered correctly/incorrectly.
    </Typography>

    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Acknowledgements
    </Typography>
    <Typography>
      Big thanks to Jake Thompson for <Link href="https://github.com/wjakethompson/taylor">compiling Taylor Swift lyrics</Link>! This game would not have been possible without these.
    </Typography>
    <Typography>
      Special thanks to David from the <Link href="https://linux.ucla.edu">Linux Users Group at UCLA</Link> for helping me get this thing on the internet!
    </Typography>
    <Typography>
      Thanks to my friends Kim and Hannah for being my most frequent testers of early versions of this lyric guessing game.
    </Typography>
    <Typography variant="h4" sx={{textDecoration: 'underline', mt:2, mb: 2}}>
      Source Code/Contact Me
    </Typography>
    <Typography>
      The source code for this project is available at <Link href="https://github.com/EricWu2003/taylorlyricguessingrs">https://github.com/EricWu2003/taylorlyricguessingrs</Link>. It's written in javascript (React) and Rust.
    </Typography>
    <Typography>
      My name is Eric, and I'm an undergraduate student at UCLA (expected graduation in spring of 2025). I love listening to Taylor Swift (surprise, right?). If you have issues with trying to run this app
      locally, please reach out -- I'd love to hear from you! My email is eric.dianhao.wu@gmail.com. Also feel free to reach out if you have any feedback about the game, and definitely let me know
      if you encounter anything you think may be a bug!
    </Typography>
  </Box>;
}