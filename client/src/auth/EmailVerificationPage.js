import React, { useState, useEffect } from "react";
import { useSearchParams, useNavigate } from "react-router-dom";
import axios from "axios";
import {
  Container,
  Button,
  Typography,
  Alert,
  CircularProgress,
  Card,
  Box,
} from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import ErrorIcon from "@mui/icons-material/Error";

export function EmailVerificationPage() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [verificationStatus, setVerificationStatus] = useState(null); // "success" or "error"


  useEffect(() => {
    handleAutoVerify(searchParams.get("email"), searchParams.get("token"));
  }, []);

  const handleAutoVerify = async (verifyEmail, verifyToken) => {
    setError("");
    try {
      await axios.post("/auth/verify-email", {
        email: verifyEmail,
        token: verifyToken,
      });
      setVerificationStatus("success");
      setMessage("Email verified successfully!");
    } catch (err) {
      setVerificationStatus("error");
      setError(err.response?.data?.error || "Failed to verify email");
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
          <Button variant="contained" color="primary" onClick={() => navigate("/")}>
            Go to Home Page
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
        </Card>
      </Container>
    );
  }

  return (
    <Box sx={{ py: 4 }} width="100%" display="flex" justifyContent="center">
      <CircularProgress></CircularProgress>
    </Box>
  );
}
