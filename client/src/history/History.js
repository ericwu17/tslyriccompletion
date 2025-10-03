import React from "react";
import { Box, CircularProgress, Table, TableBody, TableCell,
  TableHead, TableRow
} from "@mui/material";
import QueryMenuBar from "./QueryMenuBar";
import GameTableRow from "./GameTableRow";

export default function HistoryPage() {
  const [games, setGames] = React.useState([]);
  const [isLoading, setIsLoading] = React.useState(true);

  const isMobile = window.innerWidth < 600;

  const loadingIconTable = (
    <TableRow>
      <TableCell colSpan={5}>
        <Box display="flex" flexDirection="column" alignItems="center">
          <CircularProgress />
        </Box>
      </TableCell>
    </TableRow>
  );

  return (
    <Box m={2} display="flex" flexDirection="column" alignItems="center">
      <QueryMenuBar setGames={setGames} setIsLoading={setIsLoading} />

      {!isMobile &&
        <Box maxWidth="100%">
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
              {isLoading ?
                loadingIconTable
                :
                games.map((game, index) => (
                  <GameTableRow game={game} index={index} key={game.uuid} />
                ))
              }
            </TableBody>
          </Table>
        </Box>
      }

      {isMobile &&
        <Box maxWidth="100%">
          <Table aria-label="simple table" padding="none">
            <TableHead sx={{background:"#B9D9EB"}}>
              <TableRow>
                <TableCell>
                  {/* The first box has no padding on the left */}
                  <Box>
                    <strong>Time</strong>
                  </Box>
                </TableCell>
                <TableCell>
                  <Box pl={1}>
                    <strong>Player</strong>
                  </Box>
                </TableCell>
                <TableCell align="right">
                  <Box pl={1}>
                    <strong>Guesses</strong>
                  </Box>
                </TableCell>
                <TableCell align="right">
                  <Box pl={1}>
                    <strong>Score</strong>
                  </Box>
                </TableCell>
                <TableCell align="right" />
              </TableRow>
            </TableHead>
            <TableBody>
              {isLoading ?
                loadingIconTable
                :
                games.map((game, index) => (
                  <GameTableRow game={game} index={index} key={game.uuid} />
                ))
              }
            </TableBody>
          </Table>
        </Box>
      }

    </Box>
  );
}
