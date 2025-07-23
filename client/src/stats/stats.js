import React from "react";
import {
  Box, Typography, Table,
  TableCell, TableRow, TableBody,
  TableHead, CircularProgress
} from "@mui/material";
import axios from "axios";

export default function StatsPage() {
  const [stats, setStats] = React.useState({});
  const [pageState, setPageState] = React.useState(null);

  const getStats = () => {
    setPageState("LOADING");

    axios.get(
      // eslint-disable-next-line max-len
      "/stats"
    ).then((response) => {
      let stats = response.data;
      setStats(stats);
      setPageState("LOADED");
    });
  };

  React.useEffect(() => { getStats(); }, []);

  if (pageState !== "LOADED") {
    return <CircularProgress />;
  }

  const { stats_generation_time, all_time, last_7_days, last_30_days, last_365_days } = stats;


  const StatsTableRow = ({ label, stat_key }) => {
    return (
      <TableRow>
        <TableCell>
          <Typography>
            {label}
          </Typography>
        </TableCell>
        <TableCell>
          <Typography>
            {all_time[stat_key]}
          </Typography>
        </TableCell>
        <TableCell>
          <Typography>
            {last_365_days[stat_key]}
          </Typography>
        </TableCell>
        <TableCell>
          <Typography>
            {last_30_days[stat_key]}
          </Typography>
        </TableCell>
        <TableCell>
          <Typography>
            {last_7_days[stat_key]}
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
          Taylor Swift Lyric Completion Game Statistics
        </Typography>
      </Box>

      <Typography>
        Statistics generated on {stats_generation_time} UTC.
      </Typography>

      <Table aria-label="simple table">
        <TableHead>
          <TableRow>
            <TableCell align="center">
              <Typography variant="h5">
                {" "}
              </Typography>
            </TableCell>
            <TableCell align="center">
              <Typography variant="h5">
                All Time
              </Typography>
            </TableCell>
            <TableCell align="center">
              <Typography variant="h5">
                Last 365 Days
              </Typography>
            </TableCell>
            <TableCell align="center">
              <Typography variant="h5">
                Last 30 Days
              </Typography>
            </TableCell>
            <TableCell align="center">
              <Typography variant="h5">
                Last 7 Days
              </Typography>
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          <StatsTableRow label="Games played: " stat_key="num_games" />
          <StatsTableRow label="Guesses made: " stat_key="num_guesses" />
          <StatsTableRow label="Multiple-choice guesses: " stat_key="multiple_choice_guesses" />
          <StatsTableRow label="Free-response guesses: " stat_key="free_response_guesses" />
          <StatsTableRow label="Questions skipped: " stat_key="skipped" />
          <StatsTableRow label="Lifelines earned: " stat_key="num_lifelines_earned" />
          <StatsTableRow label="Lifelines used: " stat_key="num_lifelines_used" />
        </TableBody>
      </Table>
    </Box>
  );
}