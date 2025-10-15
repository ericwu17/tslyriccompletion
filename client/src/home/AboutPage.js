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
        2025-10-03: Added lyrics for The Life of a Showgirl!
      </Typography>
      <Typography>
        2024-04-25: Added lyrics for THE TORTURED POETS DEPARTMENT!
      </Typography>
      <Typography>
        2024-03-31: Please read <Link href="/changes20240331">this note</Link> about recent updates.
      </Typography>
      <Typography>
        2023-10-27: Added 1989 songs from the vault!
      </Typography>
      <Typography>
        2023-08-19: You can now copy a list of your selected songs on the start game page.
        This list can be later used to restore your song selection.
      </Typography>
      <Typography>
        View the <Link href="/changelog">full changelog here.</Link>
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
            {" "} used for styles on this webpage.
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://reactjs.org/">
              React js
            </Link>
            {" "} as a web framework.
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://www.rust-lang.org/">
              Rust
            </Link>
            {" "} for the server code which runs the game and provides API endpoints.
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://rocket.rs/">
              Rocket
            </Link>
            {" "} to easily create a HTTP server in rust.
          </ListItem>
          <ListItem disablePadding>
            <Link href="https://dev.mysql.com/doc/refman/8.0/en/what-is-mysql.html">
              MySQL
            </Link>
            {" "} as a database to store game history.
          </ListItem>
        </List>
      </Typography>

      <Typography variant="h4" sx={{mt:2}}>
        Have ideas on how to improve this site?
      </Typography>
      <Typography>
        I am always trying to fix bugs, find and mark bad lyrics, and improve the user interface.
        Let me know through the <Link href="/feedback">feedback form</Link> {}
        if you have notice any issues, have feature requests, or suggestions!
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
        My name is Eric, and I'm an undergraduate student at UCLA (expected graduation in {" "}
        <span style={{textDecoration: "line-through"}}>
          spring of 2025
        </span>{" "}
        March 2026).
        I love listening to Taylor Swift (surprise, right?), but my other interests include
        math, computers, Rubik's Cubes, and shorthand systems. If you have any feedback for
        this site, or issues with
        running this app locally, please reach out -- I'd love to hear from you! My email is
        ericwu17 at ucla.edu.
      </Typography>
      <Typography>
        Also please use the <Link href="/feedback">feedback form</Link> if
        you encounter anything you think may be a bug.
      </Typography>
    </Box>
  );
}