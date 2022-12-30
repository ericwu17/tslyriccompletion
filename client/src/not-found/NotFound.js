import React from "react";
import { Box, Link, Typography } from "@mui/material";
import { HOME_URL } from "../navbar/Navbar";

import TaylorSwiftSorry from "./TaylorSwiftSorry.jpg";

export default function NotFound() {
  return (
    <Box
      mx={4} my={2}
      display="flex" flexDirection="column"
      alignItems="center"
    >
      <Typography variant="h3">
        Page not found!
      </Typography>
      <Box my={1} />
      <Typography>
        The subdirectory of the URL was not found. Click {}
        <Link href={HOME_URL}>
          here
        </Link> to go to the home page.
      </Typography>

      <Link href="https://www.youtube.com/watch?v=VuNIsY6JdUw&t=22s">
        <Box
          component="img"
          sx={{
            width: "100%",
            maxHeight: "60vh",
            display: "block",
            objectFit: "scale-down",
          }}
          my={3}
          alt="Sorry about that!"
          src={TaylorSwiftSorry}
        />
      </Link>
    </Box>
  );
}
