import React from "react";
import { List, Box, Typography, Link, ListItem } from "@mui/material";
import { PLAY_URL } from "../navbar/Navbar";

const MECHANICS_PAGE_URL = "/mechanics";

export default function AboutPage() {
  return (
    <Box my={2} mx={5} maxWidth="100%">
      <Typography variant="h3" sx={{mb: 2}}>
        Taylor Swift Lyric Completion Game
      </Typography>
      <Typography>
        This is a game where your goal is to guess the next line from a random Taylor Swift song!
        To start playing, press <Link href={PLAY_URL}>start game</Link> in the navbar! You can
        also use this site to browse Taylor Swift lyrics, although you might
        find the line highlighting to be distracting.
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Details on Game Mechanics
      </Typography>
      <Typography>
        For details about how this game works,
        see the <Link href={MECHANICS_PAGE_URL}>mechanics page</Link>.
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Recent Changes
      </Typography>
      <Typography>
        2023-05-18: Reduced the default size of the song list display when viewing game history.
      </Typography>
      <Typography>
        2023-05-18: Patched a bug where the '&' in "Forever & Always" caused line history pages
        to be blank.
      </Typography>
      <Typography>
        2023-05-17: Changed style of View Scores page on mobile devices.
      </Typography>
      <Typography>
        2023-05-17: Changed album name "Reputation" to "reputation".
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Technologies Used
      </Typography>
      <Typography>
        This site is built with:
        <List
          sx = {{
            listStyleType: "disc",
            pl: 2,
            "& .MuiListItem-root": {
              display: "list-item",
            },
          }}
        >
          <ListItem disablePadding>
            <Link href="https://mui.com/">
              Material UI
            </Link>
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://reactjs.org/">
              React js
            </Link>
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://www.rust-lang.org/">
              Rust
            </Link>
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://rocket.rs/">
              Rocket
            </Link>
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://dev.mysql.com/doc/refman/8.0/en/what-is-mysql.html">
              MySQL
            </Link>
          </ListItem>
        </List>
      </Typography>

      <Typography variant="h4" sx={{mt:2}}>
        Known issues/Limitations
      </Typography>
      <Box>
        <List sx={{ listStyleType: "disc", listStylePosition:"inside" }}>
          <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
            When viewing the track list of Red, I removed the song "All Too Well"
            (non-ten minute version), since it contains duplicate lyrics with the ten minute
            version. This results in a mismatch between track numbers and songs. (For example,
            Come Back Be Here is track 18 on Red (TV), but it appears as track 17 on this site).
            This mismatch also occurs when the songlist is filtered by a search.
          </ListItem>
          <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
            In the song <Link href="/song/Fearless/You All Over Me">You All Over Me</Link>,
            the line "I lived, I learned" should technically have multiple different successors
            (it can be followed by "And found out what it was to turn around" or "had you, got
            burned"). However, due to how line breaks are positioned, the game thinks that there is
            only one possible successor.
            <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
              This is an issue with determining which lines are valid
              lines as a prompt. This could possibly be solved in the future by using a more clever
              method of determining which lines are valid as a prompt. (See also: {}
              <i>Selecting the line for the question
              </i> on the <Link href={MECHANICS_PAGE_URL}> Mechanics Page</Link>.)
            </ListItem>
          </ListItem>
          <ListItem sx={{ display: "list-item", fontFamily:"arial" }}>
            There are also issues where a short line appears as the prompt, and it isn't
            possible to know which song the line came from.
          </ListItem>
        </List>
      </Box>

      <Typography variant="h4" sx={{mb: 2}}>
        Future Ideas for the Taylor Swift Lyric Completion Game?
      </Typography>
      <Typography>
        I am always trying to fix bugs and improve the user interface.
        Let me know (by <Link href="mailto:eric.dianhao.wu@gmail.com">email</Link> {}
        or by {}
        <Link href="https://github.com/EricWu2003/tslyriccompletion/issues">
          raising an issue on github
        </Link>)
        if you have any feature requests or suggestions!
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Acknowledgements
      </Typography>
      <Typography>
        Big thanks to Jake Thompson for {}
        <Link href="https://github.com/wjakethompson/taylor">compiling Taylor Swift lyrics</Link>!
        This game would not have been possible without these.
      </Typography>
      <Typography>
        Special thanks to David from the {}
        <Link href="https://linux.ucla.edu">Linux Users Group at UCLA</Link>
        {} for helping me get this thing on the internet!
      </Typography>
      <Typography>
        Thanks to my friends Kim and Hannah for being my most frequent testers of early versions of
        this lyric guessing game.
      </Typography>

      <Typography variant="h4" sx={{mt:2, mb: 2}}>
        Contact Me
      </Typography>
      <Typography>
        My name is Eric, and I'm an undergraduate student at UCLA (expected graduation in spring of
        2025). I love listening to Taylor Swift (surprise, right?), but my other interests include
        math, computers, Rubik's Cubes, and shorthand systems. If you have any feedback for
        this site, or issues with
        running this app locally, please reach out -- I'd love to hear from you! My email is
        eric.dianhao.wu@gmail.com.
      </Typography>
      <Typography>
        Also please email me if
        you encounter anything you think may be a bug (or even better, leave an issue on github)!
      </Typography>
    </Box>
  );
}