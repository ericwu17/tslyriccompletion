import React from "react";
import { Box, Table, TableBody, TableCell,
  TableContainer, TableHead, TableRow, Paper
} from "@mui/material";
import QueryMenuBar from "./QueryMenuBar";
import GameTableRow from "./GameTableRow";

export default function HistoryPage() {
  const [games, setGames] = React.useState([]);

  return (
    <Box m={2} display="flex" flexDirection="column" alignItems="center">
      <QueryMenuBar setGames={setGames} />

      <Box maxWidth="100%">
        <TableContainer component={Paper}>
          <Table aria-label="simple table">
            <TableHead sx={{background:"#B9D9EB"}}>
              <TableRow>
                <TableCell><strong>Time</strong></TableCell>
                <TableCell><strong>Player</strong></TableCell>
                <TableCell align="right"><strong>Guesses</strong></TableCell>
                <TableCell align="right"><strong>Score</strong></TableCell>
                <TableCell align="right" />
              </TableRow>
            </TableHead>
            <TableBody>
              {games.map((game, index) => (
                <GameTableRow game={game} index={index} key={game.uuid} />
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Box>

    </Box>
  );
}
