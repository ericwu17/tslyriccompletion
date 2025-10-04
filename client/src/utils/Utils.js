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

const escapeQuestionMarks = s => {
  // This function replaces question marks in 's' with the string '%253F'.
  // The purpose is so that when entered into a URL on the front end, it translates to
  // '%3F', and then this gets interpreted as '?' when the front end communicates with the back end.

  // This function is needed because of the song "Question...?"
  // and also the song "Forever & always"
  return s.replaceAll("?", "%253F").replaceAll("&", "%2526");
};
const escapeQuestionMarksSingleLevel = s => {
  // sometimes we want to escape these characters quotes only one level deep
  return s.replaceAll("?", "%3F").replaceAll("&", "%26");
};

export const generateSongHref = (album, name) => {
  return `/song/${escapeQuestionMarks(album)}/${escapeQuestionMarks(name)}`;
};

export const generateGameHref = uuid => {
  return `/history/game?id=${uuid}`;
};

export const generateLineHistoryHref = (album, song, prompt) => {
  const album_esc = escapeQuestionMarks(album);
  const song_esc = escapeQuestionMarks(song);
  const prompt_esc = escapeQuestionMarks(prompt);

  return `/history/guess?album=${album_esc}&song=${song_esc}&prompt=${prompt_esc}`;
};

export const generateLineBackendAPIHistoryHref = (album, song, prompt) => {
  const album_esc = escapeQuestionMarksSingleLevel(album);
  const song_esc = escapeQuestionMarksSingleLevel(song);
  const prompt_esc = escapeQuestionMarksSingleLevel(prompt);

  return `/history/line?album=${album_esc}&song=${song_esc}&prompt=${prompt_esc}`;
};

export const unescapeQuestionMarks = s => {
  return s.replaceAll("%3F", "?").replaceAll("%26", "&");
};


// This function uses the useMediaQuery hook to determine the user's screen size.
// Then it returns an appropriate value to use in the xs prop of Grid item
// (remember that a grid is 12 units in each row, therefore a xs of 4 means
// each row will have 3 chips)
export const getAlbumChipWidth = () => {
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
