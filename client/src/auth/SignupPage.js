import React, { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useAuth } from "./useAuth";
import {
  Container,
  Paper,
  TextField,
  Button,
  Box,
  Typography,
  Alert,
  CircularProgress,
  Link as MuiLink,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
} from "@mui/material";

export function SignupPage() {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");
  const [validationError, setValidationError] = useState("");
  const [showEmailConfirmation, setShowEmailConfirmation] = useState(false);

  const { signup, isLoading, isLoggedIn } = useAuth();
  const navigate = useNavigate();

  if (isLoggedIn) {
    navigate("/");
  }

  const validateForm = () => {
    setValidationError("");

    if (!username.trim()) {
      setValidationError("Username is required");
      return false;
    }

    if (username.length < 3) {
      setValidationError("Username must be at least 3 characters");
      return false;
    }

    if (username.length > 50) {
      setValidationError("Username must be at most 50 characters");
      return false;
    }

    const usernameRegex = /^[a-zA-Z0-9_-]+$/;
    if (!usernameRegex.test(username)) {
      setValidationError("Username can only contain letters, numbers, underscores, and hyphens");
      return false;
    }
    if (!(username.length >= 6)) {
      setValidationError("Username must be at least 6 characters");
      return false;
    }


    if (email.trim()) {
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      if (!emailRegex.test(email)) {
        setValidationError("Please enter a valid email address");
        return false;
      }
    }

    if (!password) {
      setValidationError("Password is required");
      return false;
    }

    if (password.length < 8) {
      setValidationError("Password must be at least 8 characters");
      return false;
    }

    if (password !== confirmPassword) {
      setValidationError("Passwords do not match");
      return false;
    }

    return true;
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");

    if (!validateForm()) {
      return;
    }

    // If email is blank, show confirmation dialog instead of submitting
    if (!email.trim()) {
      setShowEmailConfirmation(true);
      return;
    }

    // Otherwise proceed with signup
    await performSignup();
  };

  const performSignup = async () => {
    const result = await signup(username, email.trim(), password);
    if (result.success) {
      navigate("/play");
    } else {
      setError(result.error);
    }
  };

  const handleConfirmNoEmail = () => {
    setShowEmailConfirmation(false);
    performSignup();
  };

  const handleCancelNoEmail = () => {
    setShowEmailConfirmation(false);
  };

  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" gutterBottom align="center" sx={{ mb: 3 }}>
          Create an Account
        </Typography>

        <Typography align="center" sx={{ m: 3 }}>
          An email is not necessary to create an account, but you can leave one so that
          you can reset your password in case you forget it!
        </Typography>

        <form onSubmit={handleSubmit}>
          {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
          {validationError && <Alert severity="warning" sx={{ mb: 2 }}>{validationError}</Alert>}

          <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
            <TextField
              label="Username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="Enter a username"
            />

            <TextField
              label="Email (optional)"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="Enter your email"
            />

            <TextField
              label="Password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="At least 8 characters"
            />

            <TextField
              label="Confirm Password"
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="Re-enter your password"
            />

            <Button
              type="submit"
              variant="contained"
              color="primary"
              fullWidth
              disabled={isLoading}
              sx={{ mt: 2 }}
            >
              {isLoading ? <CircularProgress size={24} /> : "Sign Up"}
            </Button>
          </Box>
        </form>

        <Box sx={{ mt: 3, textAlign: "center" }}>
          <Typography variant="body2">
            Already have an account?{" "}
            <MuiLink component={Link} to="/login">
              Log in here
            </MuiLink>
          </Typography>
        </Box>
      </Paper>

      <Dialog
        open={showEmailConfirmation}
        onClose={handleCancelNoEmail}
        aria-labelledby="email-confirmation-title"
        aria-describedby="email-confirmation-description"
      >
        <DialogTitle id="email-confirmation-title">
          Sign Up Without Email?
        </DialogTitle>
        <DialogContent>
          <DialogContentText id="email-confirmation-description">
            Are you sure you want to sign up without providing an email?
            <strong> You will not be able to change this later!</strong> This means you won't be able to
            reset your password if you forget it.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCancelNoEmail} color="primary">
            Cancel
          </Button>
          <Button onClick={handleConfirmNoEmail} color="error" variant="contained">
            Continue Without Email
          </Button>
        </DialogActions>
      </Dialog>
    </Container>
  );
}
