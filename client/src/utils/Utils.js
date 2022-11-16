import { useSearchParams } from "react-router-dom";
import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";


export const ALBUM_ORDER = [
  "Taylor Swift", "Fearless", "Speak Now", "Red", "1989", "Reputation", "Lover",
  "folklore", "evermore", "Midnights"
];

export const ALBUM_LOGOS = {
  "Taylor Swift": "https://i.scdn.co/image/ab67616d00001e022f8c0fd72a80a93f8c53b96c",
  "Fearless": "https://i.scdn.co/image/ab67616d00001e02a48964b5d9a3d6968ae3e0de",
  "Speak Now": "https://i.scdn.co/image/ab67616d00001e02e11a75a2f2ff39cec788a015",
  "Red": "https://i.scdn.co/image/ab67616d00001e02318443aab3531a0558e79a4d",
  "1989": "https://i.scdn.co/image/ab67616d00001e02332d85510aba3eb28312cfb2",
  "Reputation": "https://i.scdn.co/image/ab67616d00001e02da5d5aeeabacacc1263c0f4b",
  "Lover": "https://i.scdn.co/image/ab67616d00001e02e787cffec20aa2a396a61647",
  "folklore": "https://i.scdn.co/image/ab67616d00001e02c288028c2592f400dd0b9233",
  "evermore": "https://i.scdn.co/image/ab67616d00001e0290fd9741e1838115cd90b3b6",
  "Midnights": "https://i.scdn.co/image/ab67616d0000b273ada1a886fc3150dc695168a7",
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
  return s.replaceAll("?", "%253F");
};

export const generateSongHref = (album, name) => {
  return `/tswift/song/${escapeQuestionMarks(album)}/${escapeQuestionMarks(name)}`;
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
