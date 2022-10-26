import AboutPage from './home/AboutPage';
import SongPage from './song/SongPage';
import NotFound from './not-found/NotFound';
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { CssBaseline } from '@mui/material';
import Navbar from './navbar/Navbar';

function App() {
  return (
    <div className="App">
      <CssBaseline />
      <Navbar />
      <BrowserRouter>
        <Routes>
          <Route exact path="/" element={<AboutPage />} />
          <Route exact path="/song" element={<SongPage />} />
          <Route path="/song/:album/:name" element={<SongPage />} />
          <Route path="/song/:album" element={<SongPage />} />
          <Route path="/songs" element={<Navigate to="/song" />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
