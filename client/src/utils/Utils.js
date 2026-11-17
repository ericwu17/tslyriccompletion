import { useSearchParams } from "react-router-dom";
import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";


export const ALBUM_ORDER = [
  "Taylor Swift", "Fearless", "Speak Now", "Red", "1989", "reputation", "Lover",
  "folklore", "evermore", "Midnights", "THE TORTURED POETS DEPARTMENT", "The Life of a Showgirl",
];

export const ALBUM_LOGOS = {
  "Taylor Swift": "https://i.scdn.co/image/ab67616d00001e022f8c0fd72a80a93f8c53b96c",
  "Fearless": "https://i.scdn.co/image/ab67616d00001e02a48964b5d9a3d6968ae3e0de",
  "Speak Now": "https://i.scdn.co/image/ab67616d0000b2730b04da4f224b51ff86e0a481",
  "Red": "https://i.scdn.co/image/ab67616d00001e02318443aab3531a0558e79a4d",
  "1989": "https://i.scdn.co/image/ab67616d0000b273612fb31cf6802a4704388cbf",
  "reputation": "https://i.scdn.co/image/ab67616d00001e02da5d5aeeabacacc1263c0f4b",
  "Lover": "https://i.scdn.co/image/ab67616d00001e02e787cffec20aa2a396a61647",
  "folklore": "https://i.scdn.co/image/ab67616d00001e02c288028c2592f400dd0b9233",
  "evermore": "https://i.scdn.co/image/ab67616d00001e0290fd9741e1838115cd90b3b6",
  "Midnights": "https://i.scdn.co/image/ab67616d0000b273ada1a886fc3150dc695168a7",
  "THE TORTURED POETS DEPARTMENT":
    "https://i.scdn.co/image/ab67616d00001e028ecc33f195df6aa257c39eaa",
  "The Life of a Showgirl": "https://i.scdn.co/image/ab67616d0000b273d7812467811a7da6e6a44902",
};

// Name of the cookie that remembers the in-progress game, so a refresh resumes it.
export const GAME_ID_COOKIE = "tsgg-game-id";

export const normalizeQuotes = string => {
  const result = string
    .replaceAll("“", "\"")
    .replaceAll("”", "\"")
    .replaceAll("‘", "'")
    .replaceAll("’", "'")
    .replaceAll("`", "'")
    .replaceAll("′", "'")
    .replaceAll("″", "\"");

  return result;
};



// A function that can use used like React.useState
// but will also store the state in the URL.
// copied from https://blog.logrocket.com/use-state-url-persist-state-usesearchparams/
export function useSearchParamsState(
  searchParamName,
  defaultValue
) {
  const [searchParams, setSearchParams] = useSearchParams();

  const acquiredSearchParam = searchParams.get(searchParamName);
  const searchParamsState = acquiredSearchParam ?? defaultValue;

  const setSearchParamsState = (newState) => {
    const next = Object.assign(
      {},
      [...searchParams.entries()].reduce(
        (o, [key, value]) => ({ ...o, [key]: value }),
        {}
      ),
      { [searchParamName]: newState }
    );
    setSearchParams(next);
  };
  return [searchParamsState, setSearchParamsState];
}

// URL-building rules:
//  - Requests to the back end should pass query values through axios's `params` option
//    (and path segments through encodeURIComponent), never by pasting raw strings into the URL.
//    Song names and lyrics contain "?" and "&" (e.g. "Question...?", "Forever & Always").
//  - Links to front end pages are built with the helpers below, which encode each value once.

const toQueryString = params => {
  return Object.entries(params)
    .map(([key, value]) => `${key}=${encodeURIComponent(value)}`)
    .join("&");
};

export const generateSongHref = (album, name) => {
  return `/song/${encodeURIComponent(album)}/${encodeURIComponent(name)}`;
};

export const generateGameHref = uuid => {
  return `/history/game?${toQueryString({ id: uuid })}`;
};

export const generateUserProfileHref = username => {
  return `/users/${encodeURIComponent(username)}`;
};

export const generateLineHistoryHref = (album, song, prompt) => {
  return `/history/guess?${toQueryString({ album, song, prompt })}`;
};

export const generateFeedbackHref = (album, songName, line) => {
  return `/feedback?${toQueryString({ album, song_name: songName, line })}`;
};

// Links generated before Nov 2026 double-escaped "?" and "&" (as "%253F" and "%2526"), so after
// the router decodes them once they still read "%3F" and "%26". Apply this to values read from
// song and line-history URLs so those old links (bookmarks, shared links) keep working.
// It is a no-op for current links, and no song name or lyric contains a literal "%".
export const decodeLegacyEscapes = s => {
  return s.replaceAll("%3F", "?").replaceAll("%26", "&");
};


// This function uses the useMediaQuery hook to determine the user's screen size.
// Then it returns an appropriate value to use in the xs prop of Grid item
// (remember that a grid is 12 units in each row, therefore a xs of 4 means
// each row will have 3 chips)
export const useAlbumChipWidth = () => {
  const theme = useTheme();
  const isMediumScreen = useMediaQuery(theme.breakpoints.up("md"));
  const isLargeScreen = useMediaQuery(theme.breakpoints.up("lg"));
  return isMediumScreen ? ( isLargeScreen ? 3 : 4 ) : 6;
};

// eslint-disable-next-line max-len
// https://stackoverflow.com/questions/4817029/whats-the-best-way-to-detect-a-touch-screen-device-using-javascript
export function isTouchDevice() {
  return (("ontouchstart" in window) ||
     (navigator.maxTouchPoints > 0) ||
     (navigator.msMaxTouchPoints > 0));
}
