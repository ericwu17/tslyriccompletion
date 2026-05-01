import React, { createContext, useState, useEffect, useCallback } from "react";
import axios from "axios";

export const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [token, setToken] = useState(null);
  const [userProfile, setUserProfile] = useState(null);

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  const setProfileAndTokenFromToken = (token) => {
    axios.defaults.headers.common["Authorization"] = `Bearer ${token}`;

    // Make a request to the profile page, to confirm that the server still
    // knows that we're logged in, and validates our session token
    axios.get("/auth/profile",).then((response) => {
      if (response.status != 200) {
        logoutOnFrontend();
      } else {
        setToken(token);
        setUserProfile(response.data);
      }
    });
  };


  // Load auth state from localStorage on mount
  useEffect(() => {
    const savedToken = localStorage.getItem("authToken");
    setProfileAndTokenFromToken(savedToken);
  }, []);

  const logoutOnFrontend = () => {
    // log out on front end
    setUserProfile(null);
    setToken(null);
    localStorage.removeItem("authToken");
  };

  // Add token to axios default headers when it changes
  useEffect(() => {
    axios.defaults.headers.common["Authorization"] = `Bearer ${token}`;
  }, [token]);

  const signup = useCallback(async (username, email, password) => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await axios.post("/auth/signup", {
        username,
        email,
        password,
      });

      const { token: newToken, username: returnedUsername } = response.data;

      const newUserProfile = {
        username: returnedUsername,
        email,
      };

      setToken(newToken);
      setUserProfile(newUserProfile);
      localStorage.setItem("authToken", newToken);

      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Signup failed";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const login = useCallback(async (username_or_email, password) => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await axios.post("/auth/login", {
        username_or_email,
        password,
      });

      const { token: newToken, username: returnedUsername } = response.data;

      const newUserProfile = {
        username: returnedUsername,
      };

      setToken(newToken);
      setUserProfile(newUserProfile);
      localStorage.setItem("authToken", newToken);

      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Login failed";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const logout = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      if (token) {
        try {
          await axios.post("/auth/logout", { token });
        } catch (err) {
          // Logout from backend failed, but we'll still clear local state
          // eslint-disable-next-line
          console.error("Backend logout failed:", err);
        }
      }

      logoutOnFrontend();

      return { success: true };
    } catch (err) {
      const errorMsg = err.message || "Logout failed";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, [token]);

  const requestEmailVerification = useCallback(async (email) => {
    setIsLoading(true);
    setError(null);
    try {
      await axios.post("/auth/verify-email-request", { email });
      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Failed to send verification email";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const verifyEmail = useCallback(async (email, token) => {
    setIsLoading(true);
    setError(null);
    try {
      await axios.post("/auth/verify-email", { email, token });
      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Failed to verify email";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const requestPasswordReset = useCallback(async (email) => {
    setIsLoading(true);
    setError(null);
    try {
      await axios.post("/auth/password-reset-request", { email });
      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Failed to send password reset email";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const resetPassword = useCallback(async (email, token, newPassword) => {
    setIsLoading(true);
    setError(null);
    try {
      await axios.post("/auth/reset-password", {
        email,
        token,
        new_password: newPassword
      });
      return { success: true };
    } catch (err) {
      const errorMsg = err.response?.data?.error || "Failed to reset password";
      setError(errorMsg);
      return { success: false, error: errorMsg };
    } finally {
      setIsLoading(false);
    }
  }, []);

  const isLoggedIn = !!token && !!userProfile;

  return (
    <AuthContext.Provider
      value={{
        userProfile,
        token,
        isLoggedIn,
        isLoading,
        error,
        signup,
        login,
        logout,
        requestEmailVerification,
        verifyEmail,
        requestPasswordReset,
        resetPassword,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}
