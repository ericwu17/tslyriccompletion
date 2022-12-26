import React from "react";
import {
  Box, Select , MenuItem, Typography, FormControl,
  Divider, Checkbox, TextField, Tooltip
} from "@mui/material";
import axios from "axios";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import ArrowForwardIcon from "@mui/icons-material/ArrowForward";
import { useSearchParamsState } from "../utils/Utils";

// maximum number of games to show on one page
const PAGE_SIZE_LIMIT = 300;

export default function QueryMenuBar({setGames}) {
  const [sortBy, setSortBy] = useSearchParamsState("sortBy", "score");
  const [includeNameless, setIncludeNameless] = useSearchParamsState("includeNameless", "true");
  const [searchString, setSearchString] = useSearchParamsState("search", "");
  const [pageNum, setPageNum] = useSearchParamsState("page_num", "1");

  // We have the limit here so that someone could manually input the page
  // size into the URL, although the interface officially only supports limits of 20.
  let [limit, setLimit] = useSearchParamsState("limit", "20");

  if (isNaN(parseInt(limit))) {
    setLimit("20");
  }
  else if (parseInt(limit) > PAGE_SIZE_LIMIT) {
    limit = PAGE_SIZE_LIMIT.toString();
    setLimit(PAGE_SIZE_LIMIT.toString());
  }


  const toggleIncludeNameless = () => {
    if (includeNameless === "true") {
      setIncludeNameless("false");
    } else {
      setIncludeNameless("true");
    }
  };

  const refetchGames = () => {
    axios.get(
      // eslint-disable-next-line max-len
      `/history/all?sort=${sortBy}&search=${searchString}&include_nameless=${includeNameless}&page_num=${pageNum}&limit=${limit}`
    ).then((response) => {
      let games = response.data;
      setGames(games);
    });
  };

  const changePage = delta => {
    const currPage = parseInt(pageNum);
    if (isNaN(currPage)) {
      setPageNum("1");
      return;
    }
    const newPage = currPage + delta;
    if (newPage < 1) {
      setPageNum("1");
      return;
    }
    setPageNum(newPage.toString());
  };

  // These two useEffects handle calling refetchGames().
  // We want to call it immediately if a user changes a selected parameter,
  // but we want a small cooldown if the user is typing (since typing generates many updates)
  React.useEffect(() => {
    const timer = setTimeout(() => {
      refetchGames();
    }, 500);

    return () => clearTimeout(timer);
  }, [searchString]);

  React.useEffect(() => {
    refetchGames();
  }, [sortBy, includeNameless, pageNum]);

  return (
    <Box
      m={1} p={1}
      display="flex" flexDirection="column"
      alignItems="center" justifyContent="center"
      sx={{background:"#B9D9EB", borderRadius: 4}}
    >
      <Box
        display="flex" width="100%"
        flexWrap="wrap"
        alignItems="center" justifyContent="center"
      >
        <Box m={1} display="flex">
          <Tooltip title="Previous Page">
            <ArrowBackIcon onClick={() => changePage(-1)}/>
          </Tooltip>
        </Box>

        <Divider orientation="vertical" flexItem />

        <Box display="flex" alignItems="center" justifyContent="center" m={1}>
          <Typography mr={0.5}>
            Sort By:
          </Typography>
          <FormControl sx={{ width: 170 }} size="small">
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
        </Box>

        <Divider orientation="vertical" flexItem />

        <Box display="flex" alignItems="center" justifyContent="center">
          <Checkbox
            checked={includeNameless === "true"}
            onClick={toggleIncludeNameless}
          />
          <Box mr={1}>
            <Typography>
              Show anonymous games
            </Typography>
          </Box>
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

        <Divider orientation="vertical" flexItem />

        <Box m={1} display="flex">
          <Tooltip title="Next Page">
            <ArrowForwardIcon onClick={() => changePage(1)}/>
          </Tooltip>
        </Box>
      </Box>
      <Divider orientation="horizontal" flexItem/>
      <Typography>
        Page {pageNum}
      </Typography>
    </Box>
  );
}