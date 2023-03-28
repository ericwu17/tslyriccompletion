import React  from "react";
import { Divider, Box, Typography, Link, IconButton } from "@mui/material";
import GitHubIcon from "@mui/icons-material/GitHub";
import EmailIcon from "@mui/icons-material/Email";

export function Footer() {
  return (
    <Box className="footer">
      <Divider />
      <Box p={1} display="flex">
        <Box flexGrow={1}>
          <Typography variant="subtitle2">
            Taylor Swift Lyric Completion Game by {}
            <Link href="mailto:eric.dianhao.wu@gmail.com">
              Eric Wu
            </Link>.
          </Typography>
          <Typography variant="subtitle2">
            This site is licensed under {}
            <Link href="https://www.gnu.org/licenses/gpl-3.0.en.html">
              GPLv3
            </Link>.
          </Typography>
          <Typography variant="subtitle2">
            Source code for this site available {}
            <Link href="https://github.com/EricWu2003/tslyriccompletion">
              here
            </Link> (GitHub link).
          </Typography>
        </Box>
        <IconButton href="mailto:eric.dianhao.wu@gmail.com">
          <EmailIcon fontSize="large" />
        </IconButton>
        <IconButton href="https://github.com/EricWu2003">
          <GitHubIcon fontSize="large" />
        </IconButton>
      </Box>
    </Box>
  );
}