import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import HomeIcon from "@mui/icons-material/Home";
import MenuIcon from "@mui/icons-material/Menu";
import { Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";

// This magic number was chosen to try and get the navbar to look good at all widths.
// The main concern is that the text on buttons shouldn't need to wrap on multiple lines.
// When text starts wrapping, that's when we need to switch to mobile mode.
const MOBILE_WIDTH = 770;

const HOME_URL = "/tswift";
const PLAY_URL = "/tswift/play";
const VIEW_SCORES_URL = "/tswift/history";
const VIEW_LYRICS_URL = "/tswift/song";

export function getWindowSize() {
  const {innerWidth, innerHeight} = window;
  return {innerWidth, innerHeight};
}

export default function Navbar() {
  const [windowSize, setWindowSize] = React.useState(getWindowSize());
  const [hamburgerMenuIsOpen, setHamburgerMenuIsOpen] = React.useState(false);


  React.useEffect(() => {
    function handleWindowResize() {
      setWindowSize(getWindowSize());
    }
    window.addEventListener("resize", handleWindowResize);

    return () => {
      window.removeEventListener("resize", handleWindowResize);
    };
  }, []);

  let toolbar;
  if (windowSize.innerWidth > MOBILE_WIDTH) {
    toolbar = (
      <Toolbar>
        <IconButton
          size="large"
          edge="start"
          color="inherit"
          aria-label="menu"
          sx={{ mr: 2 }}
          onClick={() => {window.location.href=HOME_URL;}}
        >
          <HomeIcon />
        </IconButton>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          Taylor Swift Lyric Completion Game
        </Typography>
        <Button
          color="inherit"
          onClick={() => {window.location.href=PLAY_URL;}}
        >
          Start Game
        </Button>
        <Button
          color="inherit"
          onClick={() => {window.location.href=VIEW_SCORES_URL;}}
        >
          View Scores
        </Button>
        <Button
          color="inherit"
          onClick={() => {window.location.href=VIEW_LYRICS_URL;}}
        >
          View Lyrics
        </Button>
      </Toolbar>
    );
  } else {
    toolbar = (
      <Toolbar>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          TS Lyric Completion
        </Typography>
        <IconButton
          size="large"
          edge="start"
          color="inherit"
          aria-label="menu"
          sx={{ mr: 2 }}
          onClick={() => {setHamburgerMenuIsOpen(true);}}
        >
          <MenuIcon />
        </IconButton>
      </Toolbar>
    );
  }


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
      <Drawer
        anchor="top"
        open={hamburgerMenuIsOpen}
        onClose={() => setHamburgerMenuIsOpen(false)}
      >
        <HamburgerMenu />
      </Drawer>
    </Box>
  );
}

function HamburgerMenu() {
  return (
    <Box
      sx={{ width: "auto", background: "#3874CB", color:"white"}}
      role="presentation"
    >
      <List>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = HOME_URL}>
            <ListItemText sx={{flexGrow:0, marginRight:1}}>
              Home
            </ListItemText>
            <ListItemIcon>
              <HomeIcon sx={{ color: "white" }}/>
            </ListItemIcon>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = PLAY_URL}>
            <ListItemText>
              Start Game
            </ListItemText>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = VIEW_SCORES_URL}>
            <ListItemText>
              View Scores
            </ListItemText>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = VIEW_LYRICS_URL}>
            <ListItemText>
              View Lyrics
            </ListItemText>
          </ListItemButton>
        </ListItem>
      </List>
    </Box>
  );
}
