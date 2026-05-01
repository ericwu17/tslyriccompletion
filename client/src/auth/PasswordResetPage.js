import React, { useState, useEffect } from "react";
import { useSearchParams, useNavigate } from "react-router-dom";
import axios from "axios";
import {
  Container,
  Paper,
  TextField,
  Button,
  Box,
  Typography,
  Alert,
  CircularProgress,
  Card,
} from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import ErrorIcon from "@mui/icons-material/Error";

export function PasswordResetPage() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const [step, setStep] = useState("request"); // "request" or "reset"
  const [identifier, setIdentifier] = useState(searchParams.get("email") || "");
  const [email, setEmail] = useState(searchParams.get("email") || "");
  const [token, setToken] = useState(searchParams.get("token") || "");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [resetStatus, setResetStatus] = useState(null); // "success" or "error"

  // Auto-load reset form if token and email are in URL
  useEffect(() => {
    if (searchParams.get("token") && searchParams.get("email")) {
      setStep("reset");
    }
  }, [searchParams]);

  const handleRequestReset = async (e) => {
    e.preventDefault();
    setError("");
    setMessage("");

    if (!identifier.trim()) {
      setError("Username or email is required");
      return;
    }

    setIsLoading(true);
    try {
      await axios.post("/auth/password-reset-request", { identifier: identifier.trim() });
      setMessage("Password reset email sent! Check your inbox for the reset link.");
      setStep("token-entry");
    } catch (err) {
      setError(err.response?.data?.error || "Failed to send password reset email");
    } finally {
      setIsLoading(false);
    }
  };

  const handleResetPassword = async (e) => {
    e.preventDefault();
    setError("");

    if (!email.trim()) {
      setError("Email is required");
      return;
    }

    if (!token.trim()) {
      setError("Reset token is required");
      return;
    }

    if (!newPassword) {
      setError("New password is required");
      return;
    }

    if (newPassword.length < 8) {
      setError("Password must be at least 8 characters");
      return;
    }

    if (newPassword !== confirmPassword) {
      setError("Passwords do not match");
      return;
    }

    setIsLoading(true);
    try {
      await axios.post("/auth/reset-password", {
        email,
        token,
        new_password: newPassword,
      });
      setResetStatus("success");
      setMessage("Password reset successfully! Redirecting to login...");
      setTimeout(() => {
        navigate("/auth/login");
      }, 3000);
    } catch (err) {
      setResetStatus("error");
      setError(err.response?.data?.error || "Failed to reset password");
    } finally {
      setIsLoading(false);
    }
  };

  // Show success page
  if (resetStatus === "success") {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Card sx={{ textAlign: "center", p: 3 }}>
          <CheckCircleIcon sx={{ fontSize: 60, color: "success.main", mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Password Reset!
          </Typography>
          <Typography color="textSecondary" sx={{ mb: 3 }}>
            {message}
          </Typography>
          <Button variant="contained" color="primary" onClick={() => navigate("/auth/login")}>
            Go to Login
          </Button>
        </Card>
      </Container>
    );
  }

  if (resetStatus === "error") {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Card sx={{ textAlign: "center", p: 3 }}>
          <ErrorIcon sx={{ fontSize: 60, color: "error.main", mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Reset Failed
          </Typography>
          <Alert severity="error" sx={{ mb: 3 }}>
            {error}
          </Alert>
          <Button
            variant="contained"
            color="primary"
            onClick={() => {
              setStep("request");
              setIdentifier("");
              setEmail("");
              setToken("");
              setNewPassword("");
              setConfirmPassword("");
              setError("");
              setResetStatus(null);
            }}
          >
            Try Again
          </Button>
        </Card>
      </Container>
    );
  }

  // Show forms
  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom sx={{ textAlign: "center", mb: 3 }}>
          Reset Password
        </Typography>

        {error && <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError("")}>{error}</Alert>}
        {message && <Alert severity="success" sx={{ mb: 2 }}>{message}</Alert>}

        {step === "request" ? (
          <form onSubmit={handleRequestReset}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <Typography color="textSecondary" sx={{ mb: 1 }}>
                Enter your username or email address to receive a password reset link.
              </Typography>
              <TextField
                label="Username or Email"
                value={identifier}
                onChange={(e) => setIdentifier(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <Button
                type="submit"
                variant="contained"
                color="primary"
                fullWidth
                disabled={isLoading}
                sx={{ mt: 1 }}
              >
                {isLoading ? <CircularProgress size={24} /> : "Send Reset Email"}
              </Button>
              <Button
                variant="text"
                onClick={() => navigate("/auth/login")}
              >
                Back to Login
              </Button>
            </Box>
          </form>
        ) : step === "token-entry" ? (
          <form onSubmit={handleResetPassword}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <Typography color="textSecondary" sx={{ mb: 1 }}>
                Enter the reset token from your email and your new password.
              </Typography>
              <TextField
                label="Email Address"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <TextField
                label="Reset Token"
                value={token}
                onChange={(e) => setToken(e.target.value)}
                fullWidth
                disabled={isLoading}
                placeholder="Paste the token from your email"
              />
              <TextField
                label="New Password"
                type="password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <TextField
                label="Confirm Password"
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <Button
                type="submit"
                variant="contained"
                color="primary"
                fullWidth
                disabled={isLoading}
                sx={{ mt: 1 }}
              >
                {isLoading ? <CircularProgress size={24} /> : "Reset Password"}
              </Button>
              <Button
                variant="text"
                onClick={() => {
                  setStep("request");
                  setError("");
                  setMessage("");
                }}
                disabled={isLoading}
              >
                Back to Request Reset
              </Button>
            </Box>
          </form>
        ) : (
          <form onSubmit={handleResetPassword}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <Typography color="textSecondary" sx={{ mb: 1 }}>
                Enter your new password.
              </Typography>
              <TextField
                label="Email Address"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <TextField
                label="Reset Token"
                value={token}
                onChange={(e) => setToken(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <TextField
                label="New Password"
                type="password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <TextField
                label="Confirm Password"
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                fullWidth
                disabled={isLoading}
              />
              <Button
                type="submit"
                variant="contained"
                color="primary"
                fullWidth
                disabled={isLoading}
                sx={{ mt: 1 }}
              >
                {isLoading ? <CircularProgress size={24} /> : "Reset Password"}
              </Button>
            </Box>
          </form>
        )}
      </Paper>
    </Container>
  );
}
