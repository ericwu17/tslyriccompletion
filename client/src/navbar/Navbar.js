import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import HomeIcon from "@mui/icons-material/Home";
export default function Navbar() {
  const toolbar = (
    <Toolbar>
      <IconButton
        size="large"
        edge="start"
        color="inherit"
        aria-label="menu"
        sx={{ mr: 2 }}
        onClick={() => {window.location.href="/tswift";}}
      >
        <HomeIcon />
      </IconButton>
      <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
        Taylor Swift Lyric Completion Game
      </Typography>
      <Button
        color="inherit"
        onClick={() => {window.location.href="/tswift/play";}}
      >
        Start Game
      </Button>
      <Button
        color="inherit"
        onClick={() => {window.location.href="/tswift/history";}}
      >
        View Scores
      </Button>
      <Button
        color="inherit"
        onClick={() => {window.location.href="/tswift/song";}}
      >
        View Lyrics
      </Button>
    </Toolbar>
  );

  return (
    <Box>
      <AppBar position="fixed">
        {toolbar}
      </AppBar>

      {/* We render the toolbar twice: once in the AppBar and once in the page,
       so that the content of the page is not hidden beneath the AppBar.
       */}
      <Box visibility="hidden">
        {toolbar}
      </Box>
    </Box>
  );
}