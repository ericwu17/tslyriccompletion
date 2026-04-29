import React, { useState } from "react";
import { useNavigate, useLocation, Link } from "react-router-dom";
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
} from "@mui/material";

export function LoginPage() {
  const [usernameOrEmail, setUsernameOrEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [validationError, setValidationError] = useState("");

  const { login, isLoading } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();

  const validateForm = () => {
    setValidationError("");

    if (!usernameOrEmail.trim()) {
      setValidationError("Username or email is required");
      return false;
    }

    if (!password) {
      setValidationError("Password is required");
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

    const result = await login(usernameOrEmail, password);
    if (result.success) {
      // Redirect to previous page or home
      const from = location.state?.from?.pathname || "/play";
      navigate(from);
    } else {
      setError(result.error);
    }
  };

  return (
    <Container maxWidth="sm" sx={{ py: 4 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Typography variant="h4" gutterBottom align="center" sx={{ mb: 3 }}>
          Log In
        </Typography>

        <form onSubmit={handleSubmit}>
          {error && <Alert severity="error" sx={{ mb: 2 }}>{error}</Alert>}
          {validationError && <Alert severity="warning" sx={{ mb: 2 }}>{validationError}</Alert>}

          <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
            <TextField
              label="Username or Email"
              value={usernameOrEmail}
              onChange={(e) => setUsernameOrEmail(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="Enter your username or email"
            />

            <TextField
              label="Password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              disabled={isLoading}
              fullWidth
              placeholder="Enter your password"
            />

            <Button
              type="submit"
              variant="contained"
              color="primary"
              fullWidth
              disabled={isLoading}
              sx={{ mt: 2 }}
            >
              {isLoading ? <CircularProgress size={24} /> : "Log In"}
            </Button>
          </Box>
        </form>

        <Box sx={{ mt: 3, textAlign: "center" }}>
          <Typography variant="body2">
            Don't have an account?{" "}
            <MuiLink component={Link} to="/signup">
              Sign up here
            </MuiLink>
          </Typography>
        </Box>
      </Paper>
    </Container>
  );
}
