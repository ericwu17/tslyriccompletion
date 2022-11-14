import React from "react";
import {
  Box, Select , MenuItem, Typography, FormControl,
  Divider, Checkbox, TextField
} from "@mui/material";
import axios from "axios";
import { useSearchParamsState } from "../utils/Utils";


export default function QueryMenuBar({setGames}) {
  const [sortBy, setSortBy] = useSearchParamsState("sortBy", "score");
  const [includeNameless, setIncludeNameless] = useSearchParamsState("includeNameless", "true");
  const [searchString, setSearchString] = useSearchParamsState("search", "");

  const toggleIncludeNameless = () => {
    if (includeNameless === "true") {
      setIncludeNameless("false");
    } else {
      setIncludeNameless("true");
    }
  };

  const refetchGames = () => {
    axios.get(
      `/history/all?sort=${sortBy}&search=${searchString}&include_nameless=${includeNameless}`
    ).then((response) => {
      let games = response.data;
      setGames(games);
    });
  };

  React.useEffect(() => {
    refetchGames();
  }, [sortBy, includeNameless, searchString]);

  return (
    <Box
      m={1} p={1}
      display="flex" width="100%"
      alignItems="center" justifyContent="center"
      sx={{background:"#B9D9EB", borderRadius: 4}}
    >
      <Typography>
        Sort By:
      </Typography>
      <FormControl sx={{ m: 1, width: 170 }} size="small">
        <Select
          value={sortBy}
          onChange={e => {setSortBy(e.target.value);}}
        >
          <MenuItem value="score">
            Highest Score First
          </MenuItem>
          <MenuItem value="start_time">
            Most Recent First
          </MenuItem>
        </Select>
      </FormControl>

      <Divider orientation="vertical" flexItem />

      <Checkbox
        checked={includeNameless === "true"}
        onClick={toggleIncludeNameless}
      />
      <Box mr={1}>
        <Typography>
          Show anonymous games
        </Typography>
      </Box>
      <Divider orientation="vertical" flexItem />
      <Box mx={1}>
        <TextField
          placeholder="Search player name..."
          value={searchString}
          onChange={e => {setSearchString(e.target.value);}}
          onKeyDown={e => {
            if (e.key === "Enter") {
              refetchGames();
            }
          }}
          onSubmit={refetchGames}
          size="small"
        />
      </Box>
    </Box>
  );
}