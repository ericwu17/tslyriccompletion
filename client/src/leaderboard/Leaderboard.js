import React from "react";
import {
  Box, Typography, Table,
  TableCell, TableRow, TableBody,
  TableHead, CircularProgress, Link, IconButton
} from "@mui/material";
import OpenInNewIcon from "@mui/icons-material/OpenInNew";
import { PlayerNameDisplay } from "../history/GameTableRow";
import axios from "axios";

export default function LeaderboardPage() {
  const [leaderboardData, setLeaderboardData] = React.useState(null);
  const [pageState, setPageState] = React.useState(null);

  const getLeaderboard = () => {
    setPageState("LOADING");

    axios.get("/leaderboard")
      .then((response) => {
        setLeaderboardData(response.data);
        setPageState("LOADED");
      })
      .catch((error) => {
        // eslint-disable-next-line no-console
        console.error("Error fetching leaderboard:", error);
        setPageState("ERROR");
      });
  };

  React.useEffect(() => { getLeaderboard(); }, []);

  if (pageState === "LOADING") {
    return <CircularProgress />;
  }

  if (pageState === "ERROR") {
    return (
      <Box display="flex" flexDirection="column" alignItems="center" width="100%">
        <Typography variant="h3" sx={{ mb: 2 }}>
          Error Loading Leaderboard
        </Typography>
      </Box>
    );
  }

  if (pageState !== "LOADED" || !leaderboardData) {
    return <CircularProgress />;
  }

  const leaderboard = leaderboardData;
  const now = new Date();
  const monthName = now.toLocaleString("default", { month: "long" });
  const year = now.getFullYear();
  const numPlayers = leaderboard.length;

  const LeaderboardRow = ({ index, entry }) => {
    const rowFontWeight = index === 0 ? "bold" : "normal";

    let rank = index + 1;
    const medals = ["🥇", "🥈", "🥉"];
    if (index < medals.length) {
      rank = medals[index];
    }

    return (
      <TableRow>
        <TableCell align="center" sx={{ fontWeight: rowFontWeight }}>
          <Typography>
            {rank}
          </Typography>
        </TableCell>
        <TableCell align="center" sx={{ fontWeight: rowFontWeight }}>
          <PlayerNameDisplay username={entry.username}/>
        </TableCell>
        <TableCell align="center" sx={{ fontWeight: rowFontWeight }}>
          <Box display="flex" alignItems="center" justifyContent="center" gap={1}>
            <Typography>
              {entry.best_score}
            </Typography>
            <IconButton
              href={`/history/game?id=${entry.best_game_uuid}`}
              size="small"
              sx={{ padding: 0 }}
              title="View game"
            >
              <OpenInNewIcon fontSize="small" />
            </IconButton>
          </Box>
        </TableCell>
        <TableCell align="center" sx={{ fontWeight: rowFontWeight }}>
          <Typography>
            {entry.num_games}
          </Typography>
        </TableCell>
      </TableRow>
    );
  };

  return (
    <Box
      display="flex"
      flexDirection="column"
      alignItems="center"
      width="fit-content"
      minWidth="100%"
    >
      <Box my={1} mx={1}>
        <Typography variant="h3" sx={{ mb: 2 }}>
          TSLC {monthName} {year} Leaderboard
        </Typography>
      </Box>

      <Typography sx={{ mb: 2 }}>
        {numPlayers} {numPlayers === 1 ? "player" : "players"} on the leaderboard for this month.
      </Typography>

      {leaderboard.length > 0 ? (
        <Table aria-label="leaderboard table">
          <TableHead>
            <TableRow>
              <TableCell align="center">
                <Typography variant="h6">
                  Rank
                </Typography>
              </TableCell>
              <TableCell align="center">
                <Typography variant="h6">
                  Player
                </Typography>
              </TableCell>
              <TableCell align="center">
                <Typography variant="h6">
                  Best Score
                </Typography>
              </TableCell>
              <TableCell align="center">
                <Typography variant="h6">
                  Games Played
                </Typography>
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {leaderboard.map((entry, index) => (
              <LeaderboardRow key={index} index={index} entry={entry} />
            ))}
          </TableBody>
        </Table>
      ) : (
        <Typography>No players on the leaderboard yet for {monthName} {year}.</Typography>
      )}

      <Box mt={4} mb={2}>
        <Typography>
          View the{" "}
          <Link href="/history" sx={{ cursor: "pointer" }}>
            game history and scores page
          </Link>
          {" "}for more details about games.
        </Typography>
      </Box>
    </Box>
  );
}
