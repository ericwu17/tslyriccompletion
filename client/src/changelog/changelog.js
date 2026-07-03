import React from "react";
import {
  Box,
  Link,
  List,
  ListItem,
  Typography,
} from "@mui/material";


const CHANGELOG_ENTRIES = [
  (
    <Typography key="2025-10-24">
      2026-05-09: Introduced user accounts! You can now sign up for a user account.
      More features
      (such as saved songlists for starting games, a high scores page, and personal game history)
      will be coming soon.
    </Typography>
  ),
  (
    <Typography key="2025-10-24">
      2025-10-24: Added a page for Taylor Swift songs that are not romantic: {" "}
      <Link href="/not-romantic">/not-romantic</Link>
    </Typography>
  ),
  (
    <Typography key="2025-10-03">
      2025-10-03: Added lyrics for The Life of a Showgirl!
    </Typography>
  ),
  (
    <Typography key="2024-04-25">
      2024-04-25: Added lyrics for THE TORTURED POETS DEPARTMENT!
    </Typography>
  ),
  (
    <Typography key="2024-03-31">
      2024-03-31: Please read <Link href="/changes20240331">this note</Link>.
      Made the following updates:
      <List
        sx={{
          listStyleType: "disc",
          pl: 4,
          "& .MuiListItem-root": {
            display: "list-item",
          },
        }}
      >
        <ListItem disablePadding>
          Manually cleaned up all the lyrics from various songs.
        </ListItem>
      </List>
    </Typography>
  ),
  (
    <Typography key="2023-10-27">
      2023-10-27: Added 1989 songs from the vault!
    </Typography>
  ),
  (
    <Typography key="2023-08-19b">
      2023-08-19: You can now copy a list of your selected songs on the start game page.
      This list can be later used to restore your song selection.
    </Typography>
  ),
  (
    <Typography key="2023-08-19">
      2023-08-19: Pressing Enter on the start game page now does nothing. Pressing the
      "Start Game" button in the navbar will also do nothing (but will not clear your selection
      of songs).
    </Typography>
  ),
  (
    <Typography key="2023-07-08">
      2023-07-08: Added Speak Now From The Vault songs!
    </Typography>
  ),
  (
    <Typography key="2023-06-02b">
      2023-06-02: Changed the behavior of "Start Game" in Navbar. Now if the button is pressed
      while on the start game page, the game will begin immediately rather than redirecting
      the user to the back to the start game page.
    </Typography>
  ),
  (
    <Typography key="2023-06-02">
      2023-06-02: Added song "You're Losing Me" to Midnights.
    </Typography>
  ),
  (
    <Typography key="2023-05-18b">
      2023-05-18: Reduced the default size of the song list display when viewing game history.
    </Typography>
  ),
  (
    <Typography key="2023-05-18">
      2023-05-18: Patched a bug where the '&' in "Forever & Always" caused line history pages
      to be blank.
    </Typography>
  ),
  (
    <Typography key="2023-05-17b">
      2023-05-17: Changed style of View Scores page on mobile devices.
    </Typography>
  ),
  (
    <Typography key="2023-05-17">
      2023-05-17: Changed album name "Reputation" to "reputation".
    </Typography>
  ),
];

export default function Changelog() {
  return (
    <Box m={2}>
      <Typography variant="h4">
        Changelog
      </Typography>
      {CHANGELOG_ENTRIES}
    </Box>
  );
}

export function getRecentChangelogEntries() {
  return CHANGELOG_ENTRIES.slice(0, 3);  // return the first 3 elements
}