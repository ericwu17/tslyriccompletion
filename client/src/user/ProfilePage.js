import React, { useState, useEffect, useCallback } from "react";
import { useParams, useNavigate } from "react-router-dom";
import axios from "axios";
import {
  Container,
  Paper,
  Box,
  Typography,
  Button,
  CircularProgress,
  Alert,
  Card,
  CardContent,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import NotFound from "../not-found/NotFound";

export function ProfilePage() {
  const { username } = useParams();
  const navigate = useNavigate();

  const [games, setGames] = useState([]);
  const [gamesLoading, setGamesLoading] = useState(true);
  const [gamesError, setGamesError] = useState("");
  const [profile, setProfile] = useState(null);
  const [profileLoading, setProfileLoading] = useState(true);
  const [profileError, setProfileError] = useState("");
  const [page, setPage] = useState(1);
  const [userNotFound, setUserNotFound] = useState(false);
  const [hasMore, setHasMore] = useState(true);

  const fetchGames = useCallback(async (pageNum = 1) => {
    setGamesLoading(true);
    setGamesError("");
    try {
      const response = await axios.get(
        `/users/${username}/games?page_num=${pageNum}&limit=10`
      );
      const newGames = response.data;
      if (pageNum === 1) {
        setGames(newGames);
      } else {
        setGames((prev) => [...prev, ...newGames]);
      }
      setHasMore(newGames.length === 10);
    } catch (err) {
      if (err.response?.status == 404) {
        setUserNotFound(true);
      } else {
        const errorMsg =
          err.response?.data?.error || "Failed to load games";
        setGamesError(errorMsg);
      }
    } finally {
      setGamesLoading(false);
    }
  }, [username]);

  const fetchProfile = useCallback(async () => {
    setProfileLoading(true);
    setProfileError("");
    try {
      const response = await axios.get(`/users/${username}/profile`);
      setProfile(response.data);
    } catch (err) {
      if (err.response?.status == 404) {
        setUserNotFound(true);
      } else {
        const errorMsg =
          err.response?.data?.error || "Failed to load profile";
        setProfileError(errorMsg);
      }
    } finally {
      setProfileLoading(false);
    }
  }, [username]);

  useEffect(() => {
    setPage(1);
    fetchGames();
    fetchProfile();
  }, [username, fetchGames, fetchProfile]);

  if (gamesLoading) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            minHeight: "200px",
          }}
        >
          <CircularProgress />
        </Box>
      </Container>
    );
  }
  if (userNotFound) {
    return (
      <NotFound />
    );
  }

  if (gamesError) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        Error fetching games
      </Container>
    );
  }

  return (
    <Container maxWidth="md" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography
          variant="h4"
          component="h1"
          gutterBottom
          sx={{ textAlign: "center", mb: 4 }}
        >
          {username}&apos;s Profile
        </Typography>

        <Card sx={{ mb: 3 }}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Profile Summary
            </Typography>
            {profileError && (
              <Alert severity="error" sx={{ mb: 2 }}>
                {profileError}
              </Alert>
            )}
            {profileLoading ? (
              <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                <CircularProgress size={20} />
                <Typography>Loading profile...</Typography>
              </Box>
            ) : profile ? (
              <Box>
                <Typography>
                  <strong>Games played:</strong> {profile.games_played}
                </Typography>
                <Typography>
                  <strong>Guesses made:</strong> {profile.guesses_made}
                </Typography>
                <Typography>
                  <strong>Account created:</strong>{" "}
                  {new Date(profile.created_at).toLocaleDateString()}
                </Typography>
              </Box>
            ) : null}
          </CardContent>
        </Card>

        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Game History
            </Typography>
            {gamesError && (
              <Alert severity="error" sx={{ mb: 2 }}>
                {gamesError}
              </Alert>
            )}
            {games.length === 0 && !gamesLoading && !gamesError ? (
              <Typography color="textSecondary">
                No games played yet.
              </Typography>
            ) : (
              <TableContainer>
                <Table size="small">
                  <TableHead>
                    <TableRow>
                      <TableCell>Date</TableCell>
                      <TableCell>Score</TableCell>
                      <TableCell>Guesses</TableCell>
                      <TableCell>Actions</TableCell>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {games.map((game) => (
                      <TableRow key={game.uuid}>
                        <TableCell>
                          {new Date(game.start_time).toLocaleDateString()}
                        </TableCell>
                        <TableCell>
                          {game.terminal_score !== null
                            ? game.terminal_score
                            : "N/A"}
                        </TableCell>
                        <TableCell>{game.num_guesses}</TableCell>
                        <TableCell>
                          <Button
                            size="small"
                            variant="outlined"
                            onClick={() =>
                              navigate(`/history/game?id=${game.uuid}`)
                            }
                          >
                            View Details
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </TableContainer>
            )}
            {gamesLoading && (
              <Box sx={{ display: "flex", justifyContent: "center", mt: 2 }}>
                <CircularProgress size={24} />
              </Box>
            )}
            {hasMore && !gamesLoading && (
              <Box sx={{ display: "flex", justifyContent: "center", mt: 2 }}>
                <Button
                  variant="outlined"
                  onClick={() => {
                    const nextPage = page + 1;
                    setPage(nextPage);
                    fetchGames(nextPage);
                  }}
                >
                  Load More
                </Button>
              </Box>
            )}
          </CardContent>
        </Card>
      </Paper>
    </Container>
  );
}
