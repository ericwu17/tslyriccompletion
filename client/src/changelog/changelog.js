import React from "react";
import {
  Box,
  Link,
  List,
  ListItem,
  Typography,
} from "@mui/material";


export default function Changelog() {
  return (
    <Box m={2}>
      <Typography variant="h4">
        Changelog
      </Typography>

      <Typography>
        2024-04-25: Added lyrics for THE TORTURED POETS DEPARTMENT!
      </Typography>

      <Typography>
        2024-03-31: Please read <Link href="/changes20240331">this note</Link>.
        Made the following updates:
        <List
          sx = {{
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
      <Typography>
        2023-10-27: Added 1989 songs from the vault!
      </Typography>
      <Typography>
        2023-08-19: You can now copy a list of your selected songs on the start game page.
        This list can be later used to restore your song selection.
      </Typography>
      <Typography>
        2023-08-19: Pressing Enter on the start game page now does nothing. Pressing the
        "Start Game" button in the navbar will also do nothing (but will not clear your selection
        of songs).
      </Typography>
      <Typography>
        2023-07-08: Added Speak Now From The Vault songs!
      </Typography>
      <Typography>
        2023-06-02: Changed the behavior of "Start Game" in Navbar. Now if the button is pressed
        while on the start game page, the game will begin immediately rather than redirecting
        the user to the back to the start game page.
      </Typography>
      <Typography>
        2023-06-02: Added song "You're Losing Me" to Midnights.
      </Typography>
      <Typography>
        2023-05-18: Reduced the default size of the song list display when viewing game history.
      </Typography>
      <Typography>
        2023-05-18: Patched a bug where the '&' in "Forever & Always" caused line history pages
        to be blank.
      </Typography>
      <Typography>
        2023-05-17: Changed style of View Scores page on mobile devices.
      </Typography>
      <Typography>
        2023-05-17: Changed album name "Reputation" to "reputation".
      </Typography>
    </Box>
  );
}