import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import HomeIcon from "@mui/icons-material/Home";
import MenuIcon from "@mui/icons-material/Menu";
import {
  CssBaseline, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Menu, MenuItem
} from "@mui/material";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../auth/useAuth";

import Satisfaction from "../fonts/Satisfaction.ttf";
import { createTheme, ThemeProvider, } from "@mui/material/styles";


const theme = createTheme({
  typography: {
    fontFamily: "Satisfaction, cursive",
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: `
        @font-face {
          font-family: 'Satisfaction';
          font-style: normal;
          font-display: swap;
          font-weight: 400;
          src: local('Satisfaction'), 
            local('Satisfaction-Regular'), url(${Satisfaction}) format('woff2');
        }
      `,
    },
  },
});



// This magic number was chosen to try and get the navbar to look good at all widths.
// The main concern is that the text on buttons shouldn't need to wrap on multiple lines.
// When text starts wrapping, that's when we need to switch to mobile mode.
const MOBILE_WIDTH = 770;

export const HOME_URL = "/";
export const PLAY_URL = "/play";
export const VIEW_LEADERBOARD_URL = "/leaderboard";
export const VIEW_SCORES_URL = "/history";
export const VIEW_LYRICS_URL = "/song";
export const VIEW_STATS_URL = "/stats";

export function getWindowSize() {
  const { innerWidth, innerHeight } = window;
  return { innerWidth, innerHeight };
}

export default function Navbar() {
  const [windowSize, setWindowSize] = React.useState(getWindowSize());
  const [hamburgerMenuIsOpen, setHamburgerMenuIsOpen] = React.useState(false);
  const [anchorEl, setAnchorEl] = React.useState(null);
  const { isLoggedIn, userPersonalDetails, logout } = useAuth();
  const navigate = useNavigate();

  React.useEffect(() => {
    function handleWindowResize() {
      setWindowSize(getWindowSize());
    }
    window.addEventListener("resize", handleWindowResize);

    return () => {
      window.removeEventListener("resize", handleWindowResize);
    };
  }, []);

  const handleUserMenuOpen = (event) => {
    setAnchorEl(event.currentTarget);
  };

  const handleUserMenuClose = () => {
    setAnchorEl(null);
  };

  const handleLogout = async () => {
    await logout();
    handleUserMenuClose();
    navigate("/");
  };

  const goToPersonalDetails = () => {
    navigate("/personal-details");
  };

  const goToMyProfile = () => {
    navigate(`/users/${userPersonalDetails?.username}`);
  };

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
          onClick={() => { window.location.href = HOME_URL; }}
        >
          <HomeIcon />
        </IconButton>
        <ThemeProvider theme={theme}>
          <CssBaseline />
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Taylor Swift Lyric Completion Game
          </Typography>
        </ThemeProvider>
        <Button
          color="inherit"
          onClick={() => { window.location.href = PLAY_URL; }}
        >
          Play Game
        </Button>
        <Button
          color="inherit"
          onClick={() => { window.location.href = VIEW_LEADERBOARD_URL; }}
        >
          Leaderboards
        </Button>
        <Button
          color="inherit"
          onClick={() => { window.location.href = VIEW_STATS_URL; }}
        >
          Stats
        </Button>
        <Button
          color="inherit"
          onClick={() => { window.location.href = VIEW_LYRICS_URL; }}
        >
          Lyrics
        </Button>
        {isLoggedIn ? (
          <>
            <IconButton
              color="inherit"
              onClick={handleUserMenuOpen}
              title={`Logged in as ${userPersonalDetails?.username}`}
            >
              <AccountCircleIcon />
            </IconButton>
            <Menu
              anchorEl={anchorEl}
              open={Boolean(anchorEl)}
              onClose={handleUserMenuClose}
            >
              <MenuItem disabled>
                {userPersonalDetails?.username}
              </MenuItem>
              <MenuItem onClick={goToMyProfile}>
                My Profile
              </MenuItem>
              <MenuItem onClick={goToPersonalDetails}>
                Personal Details
              </MenuItem>
              <MenuItem onClick={handleLogout}>
                Logout
              </MenuItem>
            </Menu>
          </>
        ) : (
          <>
            <Button
              color="inherit"
              onClick={() => navigate("/auth/login")}
            >
              Login
            </Button>
            <Button
              color="inherit"
              onClick={() => navigate("/auth/signup")}
            >
              Sign Up
            </Button>
          </>
        )}
      </Toolbar>
    );
  } else {
    toolbar = (
      <Toolbar>
        <ThemeProvider theme={theme}>
          <CssBaseline />
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            TS Lyric Completion
          </Typography>
        </ThemeProvider>
        {isLoggedIn && (
          <IconButton
            size="large"
            color="inherit"
            onClick={handleUserMenuOpen}
            title={`Logged in as ${userPersonalDetails?.username}`}
            sx={{ mr: 1 }}
          >
            <AccountCircleIcon />
          </IconButton>
        )}
        <Menu
          anchorEl={anchorEl}
          open={Boolean(anchorEl)}
          onClose={handleUserMenuClose}
        >
          {isLoggedIn && (
            <>
              <MenuItem disabled>
                {userPersonalDetails?.username}
              </MenuItem>
              <MenuItem onClick={goToMyProfile}>
                My Profile
              </MenuItem>
              <MenuItem onClick={goToPersonalDetails}>
                Personal Details
              </MenuItem>
              <MenuItem onClick={handleLogout}>
                Logout
              </MenuItem>
            </>
          )}
        </Menu>
        <IconButton
          size="large"
          edge="start"
          color="inherit"
          aria-label="menu"
          sx={{ mr: 2 }}
          onClick={() => { setHamburgerMenuIsOpen(true); }}
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
  const { isLoggedIn, logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    navigate("/");
  };

  return (
    <Box
      sx={{ width: "auto", background: "#3874CB", color: "white" }}
      role="presentation"
    >
      <List>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = HOME_URL}>
            <ListItemText sx={{ flexGrow: 0, marginRight: 1 }}>
              Home
            </ListItemText>
            <ListItemIcon>
              <HomeIcon sx={{ color: "white" }} />
            </ListItemIcon>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = PLAY_URL}>
            <ListItemText>
              Play Game
            </ListItemText>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = VIEW_LEADERBOARD_URL}>
            <ListItemText>
              Leaderboards
            </ListItemText>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = VIEW_STATS_URL}>
            <ListItemText>
              Stats
            </ListItemText>
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton onClick={() => window.location.href = VIEW_LYRICS_URL}>
            <ListItemText>
              Lyrics
            </ListItemText>
          </ListItemButton>
        </ListItem>
        {isLoggedIn ? (
          <ListItem disablePadding>
            <ListItemButton onClick={handleLogout}>
              <ListItemText>
                Logout
              </ListItemText>
            </ListItemButton>
          </ListItem>
        ) : (
          <>
            <ListItem disablePadding>
              <ListItemButton onClick={() => navigate("/auth/login")}>
                <ListItemText>
                  Login
                </ListItemText>
              </ListItemButton>
            </ListItem>
            <ListItem disablePadding>
              <ListItemButton onClick={() => navigate("/auth/signup")}>
                <ListItemText>
                  Sign Up
                </ListItemText>
              </ListItemButton>
            </ListItem>
          </>
        )}
      </List>
    </Box>
  );
}
