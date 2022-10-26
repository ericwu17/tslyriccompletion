import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import HomeIcon from '@mui/icons-material/Home';
export default function Navbar() {
  return (
    <Box sx={{ flexGrow: 1 }} mb={10}>
      <AppBar position="fixed">
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
            onClick={() => {window.location.href="/"}}
          >
            <HomeIcon />
          </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Taylor Swift Lyric Guessing Game
          </Typography>
          <Button 
            color="inherit"
            onClick={() => {window.location.href="/about"}}
          >
            About This Site
          </Button>
          <Button 
            color="inherit"
            onClick={() => {window.location.href="/songs"}}
          >
            View Lyrics
          </Button>
        </Toolbar>
      </AppBar>
    </Box>
  );
}