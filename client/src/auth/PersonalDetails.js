import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "./useAuth";
import axios from "axios";
import {
  Container,
  Paper,
  Box,
  Typography,
  Button,
  Alert,
  Card,
  CardContent,
  Chip,
  Snackbar,
} from "@mui/material";
import VerifiedIcon from "@mui/icons-material/Verified";
import ErrorIcon from "@mui/icons-material/Error";

export function PersonalDetails() {
  const navigate = useNavigate();
  const { isLoggedIn, logout, userPersonalDetails} = useAuth();

  const data = userPersonalDetails;

  const [changeEmailMessageIsOpen, setChangeEmailDialogIsOpen] = useState(false);
  const [error, setError] = useState("");
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [isVerifyEmailLoading, setIsVerifyEmailLoading] = useState(false);

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
        <Alert severity="info">Please log in to view your personal details</Alert>
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
          Personal Details
        </Typography>

        {data && (
          <Box sx={{ display: "flex", flexDirection: "column", gap: 3 }}>
            {/* Username Card */}
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>
                  Username
                </Typography>
                <Typography variant="h6">
                  {data.username}
                </Typography>
              </CardContent>
            </Card>

            {/* Email Card */}
            {data.email &&
              <Card>
                <CardContent>
                  <Box sx={
                    {
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "flex-start",
                      mb: 1
                    }
                  }>
                    <Box>
                      <Typography color="textSecondary" gutterBottom>
                        Email Address
                      </Typography>
                      <Typography variant="h6">
                        {data.email}
                      </Typography>
                    </Box>
                    <Box>
                      {data.email_verified ? (
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

                  {!data.email_verified && (
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

            <Card>
              <Box m={2} onClick={() => setChangeEmailDialogIsOpen(!changeEmailMessageIsOpen)}>
                <Button>Change email or username</Button>
              </Box>
              {changeEmailMessageIsOpen &&
              <Box m={2}>
                <Typography>
                  To change your email or username,
                  send an email to tslyriccompletion@gmail.com, from the old email address
                  on your account.
                </Typography>
                <Typography>
                  You cannot change your email/username if you did not sign up
                  with an email address.
                </Typography>
                <Typography>
                  At some point in the future, if I feel inspired to do more work on this site,
                  I'll automate the process of changing your email/username
                  so that it's easier for users (lol).
                  This would also make it possible to change your
                  username or add an email address if you did not previously sign up with one :)
                </Typography>
              </Box>
              }
            </Card>

            {/* Action Buttons */}
            <Box sx={{ display: "flex", gap: 2, mt: 3 }}>
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
