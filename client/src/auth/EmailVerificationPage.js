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

export function EmailVerificationPage() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const [step, setStep] = useState("request"); // "request", "manual-verify", or "auto-verify"
  const [email, setEmail] = useState(searchParams.get("email") || "");
  const [token, setToken] = useState(searchParams.get("token") || "");
  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [verificationStatus, setVerificationStatus] = useState(null); // "success" or "error"

  // Auto-verify if token and email are in URL
  useEffect(() => {
    if (searchParams.get("token") && searchParams.get("email")) {
      setStep("auto-verify");
      handleAutoVerify(searchParams.get("email"), searchParams.get("token"));
    }
  }, [searchParams]);

  const handleAutoVerify = async (verifyEmail, verifyToken) => {
    setIsLoading(true);
    setError("");
    try {
      await axios.post("/auth/verify-email", {
        email: verifyEmail,
        token: verifyToken,
      });
      setVerificationStatus("success");
      setMessage("Email verified successfully! Redirecting to login...");
      setTimeout(() => {
        navigate("/auth/login");
      }, 3000);
    } catch (err) {
      setVerificationStatus("error");
      setError(err.response?.data?.error || "Failed to verify email");
    } finally {
      setIsLoading(false);
    }
  };

  const handleRequestVerification = async (e) => {
    e.preventDefault();
    setError("");
    setMessage("");

    if (!email.trim()) {
      setError("Email is required");
      return;
    }

    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(email)) {
      setError("Please enter a valid email address");
      return;
    }

    setIsLoading(true);
    try {
      await axios.post("/auth/verify-email-request", { email });
      setMessage("Verification email sent! Check your inbox for the verification link.");
      setStep("manual-verify");
    } catch (err) {
      setError(err.response?.data?.error || "Failed to send verification email");
    } finally {
      setIsLoading(false);
    }
  };

  const handleManualVerify = async (e) => {
    e.preventDefault();
    setError("");

    if (!email.trim()) {
      setError("Email is required");
      return;
    }

    if (!token.trim()) {
      setError("Verification token is required");
      return;
    }

    setIsLoading(true);
    try {
      await axios.post("/auth/verify-email", {
        email,
        token,
      });
      setVerificationStatus("success");
      setMessage("Email verified successfully! Redirecting to login...");
      setTimeout(() => {
        navigate("/auth/login");
      }, 3000);
    } catch (err) {
      setVerificationStatus("error");
      setError(err.response?.data?.error || "Failed to verify email");
    } finally {
      setIsLoading(false);
    }
  };

  // Show success/error page
  if (verificationStatus === "success") {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Card sx={{ textAlign: "center", p: 3 }}>
          <CheckCircleIcon sx={{ fontSize: 60, color: "success.main", mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Email Verified!
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

  if (verificationStatus === "error") {
    return (
      <Container maxWidth="sm" sx={{ py: 4 }}>
        <Card sx={{ textAlign: "center", p: 3 }}>
          <ErrorIcon sx={{ fontSize: 60, color: "error.main", mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Verification Failed
          </Typography>
          <Alert severity="error" sx={{ mb: 3 }}>
            {error}
          </Alert>
          <Button
            variant="contained"
            color="primary"
            onClick={() => {
              setStep("request");
              setEmail("");
              setToken("");
              setError("");
              setVerificationStatus(null);
            }}
          >
            Try Again
          </Button>
        </Card>
      </Container>
    );
  }

  // Show form
  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom sx={{ textAlign: "center", mb: 3 }}>
          Email Verification
        </Typography>

        {error && <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError("")}>{error}</Alert>}
        {message && <Alert severity="success" sx={{ mb: 2 }}>{message}</Alert>}

        {step === "request" ? (
          <form onSubmit={handleRequestVerification}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <Typography color="textSecondary" sx={{ mb: 1 }}>
                Enter your email address to receive a verification link.
              </Typography>
              <TextField
                label="Email Address"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
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
                {isLoading ? <CircularProgress size={24} /> : "Send Verification Email"}
              </Button>
            </Box>
          </form>
        ) : (
          <form onSubmit={handleManualVerify}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <Typography color="textSecondary" sx={{ mb: 1 }}>
                Enter your email and the verification token from the email.
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
                label="Verification Token"
                value={token}
                onChange={(e) => setToken(e.target.value)}
                fullWidth
                disabled={isLoading}
                placeholder="Paste the token from your email"
              />
              <Button
                type="submit"
                variant="contained"
                color="primary"
                fullWidth
                disabled={isLoading}
                sx={{ mt: 1 }}
              >
                {isLoading ? <CircularProgress size={24} /> : "Verify Email"}
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
                Back to Request Verification
              </Button>
            </Box>
          </form>
        )}
      </Paper>
    </Container>
  );
}
