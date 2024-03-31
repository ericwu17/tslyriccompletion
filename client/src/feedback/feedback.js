import React from "react";
import {
  Box,  Button,
  TextField,
  Typography,
} from "@mui/material";
import { useLocation } from "react-router-dom";
import axios from "axios";



export default function FeedbackForm() {
  const useQuery = () => new URLSearchParams(useLocation().search);
  const query = useQuery();
  const [album, setAlbum] = React.useState(query.get("album") || "");
  const [song, setSong] =  React.useState(query.get("song_name") || "");
  const [lyric, setLyric] =  React.useState(query.get("line") || "");
  const [msg, setMsg] =  React.useState("");
  const [contact, setContact] = React.useState("");

  const [hasSent, setHasSent] = React.useState(false);

  const onClickSubmit = () => {
    if (msg.length > 1000 || album.length > 100
      || song.length > 100 || lyric.length > 500 || contact.length > 200) {
      // eslint-disable-next-line
      alert("Too much feedback! Please limit each field to 100 characters, except for the message which can be 1000 characters.");
      return;
    }

    if (msg != "") {
      const feedbackObject = {
        album: album,
        song: song,
        lyric: lyric,
        message: msg,
        contact: contact
      };

      axios.post("/feedback/general", feedbackObject).then(() => {
        setHasSent(true);
      });
    }
  };

  return (
    <Box m={2}>
      {!hasSent &&
        <>
          <Typography variant="h4">
            Give feedback or report an error
          </Typography>
          <Typography variant="body1">
            Please use this form to write feedback so that I can make this lyric completion
            game better!
          </Typography>
          <Typography variant="body1">
            If giving feedback about a specific line in a song, please fill out the album, song,
            and lyric fields.
            Otherwise, only the message field needs to be completed.
          </Typography>
          <Typography variant="body1">
            If you would like me to respond to your feedback, you may leave your email and/or
            phone number in the contact info field.
          </Typography>

          <Box m={1}>
            <TextField
              onChange={event => setAlbum(event.target.value)}
              onKeyDown={null}
              label="Album (optional)"
              value={album}
              size="small"
              sx={{width: "100%"}}
            />
          </Box>

          <Box m={1}>
            <TextField
              onChange={event => setSong(event.target.value)}
              onKeyDown={null}
              label="Song (optional)"
              value={song}
              size="small"
              sx={{width: "100%"}}
            />
          </Box>

          <Box m={1}>
            <TextField
              onChange={event => setLyric(event.target.value)}
              onKeyDown={null}
              label="Lyric (optional)"
              value={lyric}
              size="small"
              sx={{width: "100%"}}
            />
          </Box>

          <Box m={1}>
            <TextField
              onChange={event => setMsg(event.target.value)}
              onKeyDown={null}
              label="Message"
              value={msg}
              sx={{width: "100%"}}
              multiline
              minRows={5}
            />
          </Box>


          <Box m={1}>
            <TextField
              onChange={event => setContact(event.target.value)}
              onKeyDown={null}
              label="Contact information (optional)"
              value={contact}
              size="small"
              sx={{width: "100%"}}
            />
          </Box>


          <Box m={1} sx={{width: "max-content", border: "2px solid green"}}>
            <Button onClick={onClickSubmit} size="large">
              Send feedback
            </Button>
          </Box>
        </>
      }
      {hasSent &&
        <>
          <Typography>
            Your feedback has been received, thank you.
          </Typography>
        </>
      }
    </Box>
  );
}