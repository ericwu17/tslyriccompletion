import './App.css';
import Homepage from './home/Homepage';
import SongPage from './song/SongPage';
import NotFound from './not-found/NotFound';
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";

function App() {
  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
          <Route exact path="/" element={<Homepage />} />
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
