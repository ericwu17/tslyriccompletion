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
