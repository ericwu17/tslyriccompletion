import React  from "react";
import AboutPage from "./home/AboutPage";
import SongPage from "./song/SongPage";
import NotFound from "./not-found/NotFound";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { CssBaseline, Box } from "@mui/material";
import Navbar from "./navbar/Navbar";
import Game from "./game/Game";
import HistoryPage from "./history/History";
import GameDetails from "./history/GameDetails";
import MechanicsPage from "./home/Mechanics";
import GuessHistory from "./history/GuessHistory";
import { Footer } from "./navbar/Footer";

function App() {
  return (
    <Box
      className="App" display="flex" flexDirection="column" height="100vh"
      // Added this sx prop because 100vh is too large on mobile devices:
      // https://stackoverflow.com/questions/37112218/css3-100vh-not-constant-in-mobile-browser
      sx={{maxHeight: "-webkit-fill-available"}}
    >
      <CssBaseline />
      <Navbar />
      <Box flexGrow={1}>
        <BrowserRouter basename="tswift">
          <Routes>
            <Route exact path="/" element={<AboutPage />} />
            <Route exact path="/mechanics" element={<MechanicsPage />} />
            <Route exact path="/play" element={<Game />} />
            <Route exact path="/song" element={<SongPage />} />
            <Route path="/song/:album/:name" element={<SongPage />} />
            <Route path="/song/:album" element={<SongPage />} />
            <Route path="/songs" element={<Navigate to="/song" />} />
            <Route path="/history" element={<HistoryPage />} />
            <Route path="/history/game" element={<GameDetails />} />
            <Route path="/history/guess" element={<GuessHistory />} />
            <Route path=":any/*" element={<NotFound />} />
          </Routes>
        </BrowserRouter>
      </Box>
      <Footer />
    </Box>
  );
}

export default App;
