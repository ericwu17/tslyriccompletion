import React  from "react";
import AboutPage from "./home/AboutPage";
import SongPage from "./song/SongPage";
import NotFound from "./not-found/NotFound";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { CssBaseline } from "@mui/material";
import Navbar from "./navbar/Navbar";
import Game from "./game/Game";
import HistoryPage from "./history/History";
import MechanicsPage from "./home/Mechanics";

function App() {
  return (
    <div className="App">
      <CssBaseline />
      <Navbar />
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
          <Route path="*" element={<NotFound />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
