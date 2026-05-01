import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "./useAuth";
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
  Chip,
  Snackbar,
} from "@mui/material";
import VerifiedIcon from "@mui/icons-material/Verified";
import ErrorIcon from "@mui/icons-material/Error";

export function ProfilePage() {
  const navigate = useNavigate();
  const { token, isLoggedIn, logout } = useAuth();

  const [profile, setProfile] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState("");
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [isVerifyEmailLoading, setIsVerifyEmailLoading] = useState(false);

  useEffect(() => {
    if (!isLoggedIn) {
      navigate("/auth/login");
      return;
    }

    const fetchProfile = async () => {
      setIsLoading(true);
      setError("");
      try {
        const response = await axios.get("/auth/profile");
        setProfile(response.data);
      } catch (err) {
        const errorMsg = err.response?.data?.error || "Failed to load profile";
        setError(errorMsg);
        if (err.response?.status === 401) {
          navigate("/auth/login");
        }
      } finally {
        setIsLoading(false);
      }
    };

    fetchProfile();
  }, [isLoggedIn, token, navigate]);

  const handleLogout = async () => {
    await logout();
    navigate("/");
  };

  const handleRequestEmailVerification = async () => {
    setIsVerifyEmailLoading(true);
    try {
      await axios.post("/auth/verify-email-request");
      setSnackbarMessage("Verification email sent! Check your inbox.");
      setSnackbarOpen(true);
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Failed to send verification email";
      setError(errorMsg);
    } finally {
      setIsVerifyEmailLoading(false);
    }
  };

  if (!isLoggedIn) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Alert severity="info">Please log in to view your profile</Alert>
      </Container>
    );
  }

  if (isLoading) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Box sx={{ display: "flex", justifyContent: "center", alignItems: "center", minHeight: "200px" }}>
          <CircularProgress />
        </Box>
      </Container>
    );
  }

  if (error) {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Alert severity="error">{error}</Alert>
      </Container>
    );
  }

  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom sx={{ textAlign: "center", mb: 4 }}>
          My Profile
        </Typography>

        {profile && (
          <Box sx={{ display: "flex", flexDirection: "column", gap: 3 }}>
            {/* Username Card */}
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>
                  Username
                </Typography>
                <Typography variant="h6">
                  {profile.username}
                </Typography>
              </CardContent>
            </Card>

            {/* Email Card */}
            {profile.email &&
              <Card>
                <CardContent>
                  <Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", mb: 1 }}>
                    <Box>
                      <Typography color="textSecondary" gutterBottom>
                        Email Address
                      </Typography>
                      <Typography variant="h6">
                        {profile.email}
                      </Typography>
                    </Box>
                    <Box>
                      {profile.email_verified ? (
                        <Chip
                          icon={<VerifiedIcon />}
                          label="Verified"
                          color="success"
                          variant="outlined"
                        />
                      ) : (
                        <Chip
                          icon={<ErrorIcon />}
                          label="Not Verified"
                          color="error"
                          variant="outlined"
                        />
                      )}
                    </Box>
                  </Box>

                  {!profile.email_verified && (
                    <Button
                      variant="contained"
                      color="warning"
                      size="small"
                      onClick={handleRequestEmailVerification}
                      disabled={isVerifyEmailLoading}
                      sx={{ mt: 2 }}
                    >
                      {isVerifyEmailLoading ? "Sending..." : "Verify Email"}
                    </Button>
                  )}
                </CardContent>
              </Card>
            }

            {/* Action Buttons */}
            <Box sx={{ display: "flex", gap: 2, mt: 3 }}>
              <Button
                variant="contained"
                color="primary"
                fullWidth
                onClick={() => navigate("/play")}
              >
                Back to Game
              </Button>
              <Button
                variant="outlined"
                color="error"
                fullWidth
                onClick={handleLogout}
              >
                Log Out
              </Button>
            </Box>
          </Box>
        )}
      </Paper>

      <Snackbar
        open={snackbarOpen}
        autoHideDuration={6000}
        onClose={() => setSnackbarOpen(false)}
        message={snackbarMessage}
      />
    </Container>
  );
}
