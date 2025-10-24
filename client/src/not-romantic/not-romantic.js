import React from "react";
import {
  Box,
  Link,
  Typography,
} from "@mui/material";
import { ALBUM_LOGOS, ALBUM_ORDER, generateSongHref } from "../utils/Utils";

const NOT_ROMANTIC_SONGS_DATA = [
  {
    "album": "Taylor Swift",
    "name": "A Place In This World",
    "description": " is about finding your place in the world as a young person.",
  },
  {
    "album": "Taylor Swift",
    "name": "The Outside",
    "description": " is about feeling lonely.",
  },
  {
    "album": "Taylor Swift",
    "name": "Tied Together With A Smile",
    "description": " is about her friend with an eating disorder.",
  },
  {
    "album": "Taylor Swift",
    "name": "Mary's Song",
    // eslint-disable-next-line max-len
    "description": " is about Taylor's neighbors, an old couple who had known each other since they were kids.",
  },
  {
    "album": "Taylor Swift",
    "name": "I'm Only Me When I'm With You",
    "description": " is about a redhead named Abigail that Taylor sat next to in class.",
  },
  {
    "album": "Fearless",
    "name": "Fifteen",
    "description": " is about her freshman year in high school.",
  },
  {
    "album": "Fearless",
    "name": "The Best Day",
    "description": " is about her mother.",
  },
  {
    "album": "Fearless",
    "name": "Change",
    "description": " is an underdog story.",
  },
  {
    "album": "Speak Now",
    "name": "Never Grow Up",
    "description": " is for a baby of one of Taylor's friends.",
  },
  {
    "album": "Speak Now",
    "name": "Innocent",
    "description": " is about accepting and learning from one's mistakes.",
  },
  {
    "album": "Speak Now",
    "name": "Castles Crumbling",
    "description": " is about the fear of losing one's reputation, or the pressure of fame.",
  },
  {
    "album": "Red",
    "name": "22",
    "description": " is about partying and being somewhat carefree when you're 22.",
  },
  {
    "album": "Red",
    "name": "The Lucky One",
    "description": " questions whether it's lucky to get famous.",
  },
  {
    "album": "Red",
    "name": "Ronan",
    "description": " is about boy who died of cancer at age 4.",
  },
  {
    "album": "Red",
    "name": "Nothing New",
    // eslint-disable-next-line max-len
    "description": " is about aging, and the insecurity of being replaced by a younger counterpart.",
  },
  {
    "album": "Red",
    "name": "Forever Winter",
    "description": " is about a friend with mental health struggles.",
  },
  {
    "album": "Red",
    "name": "Run",
    "description": " is primarily about getting out of one's current situation.",
  },
  {
    "album": "1989",
    "name": "Welcome To New York",
    "description": " is about being optimistic about a new city.",
  },
  {
    "album": "1989",
    "name": "Blank Space",
    "description": " is about the media portraying Taylor as a serial dater.",
  },
  {
    "album": "1989",
    "name": "Shake It Off",
    "description": " is for the haters.",
  },
  {
    "album": "1989",
    "name": "Bad Blood",
    "description": " is about a fallout.",
  },
  {
    "album": "1989",
    "name": "I Know Places",
    // eslint-disable-next-line max-len
    "description": " is about hiding from paparazzi, and the challenges of having a relationship under the public eye.",
  },
  {
    "album": "reputation",
    "name": "I Did Something Bad",
    "description": " is defiance from backlash from the media.",
  },
  {
    "album": "reputation",
    "name": "Look What You Made Me Do",
    "description": " is about betrayal and drama.",
  },
  {
    "album": "reputation",
    "name": "This Is Why We Can't Have Nice Things",
    "description": " is also about betrayal and drama.",
  },
  {
    "album": "Lover",
    "name": "The Man",
    "description": " is about double standards between genders.",
  },
  {
    "album": "Lover",
    "name": "Miss Americana & The Heartbreak Prince",
    "description": " is ostensibly about love, but it's a metaphor for politics.",
  },
  {
    "album": "Lover",
    "name": "Soon You'll Get Better",
    "description": " is about her sick mother.",
  },
  {
    "album": "Lover",
    "name": "You Need To Calm Down",
    "description": " is about online hate and gay pride.",
  },
  {
    "album": "folklore",
    "name": "the last great american dynasty",
    "description": " is about the socialite Rebekah Harkness.",
  },
  {
    "album": "folklore",
    "name": "mirrorball",
    "description": " is about changing oneself to please people.",
  },
  {
    "album": "folklore",
    "name": "seven",
    "description": " is Taylor reminiscing on her childhood in Pennsylvania.",
  },
  {
    "album": "folklore",
    "name": "this is me trying",
    "description": " is about depression.",
  },
  {
    "album": "folklore",
    "name": "mad woman",
    "description": " is about gender norms for expressing anger.",
  },
  {
    "album": "folklore",
    "name": "epiphany",
    // eslint-disable-next-line max-len
    "description": " is about living in a world of chaos (covid 19), and compares healthcare workers to soldiers in war.",
  },
  {
    "album": "folklore",
    "name": "the lakes",
    "description": " expresses a desire for solitude and escape.",
  },
  {
    "album": "evermore",
    "name": "no body no crime",
    "description": " is a fictional story about avenging a murder.",
  },
  {
    "album": "evermore",
    "name": "dorothea",
    "description": " reminisces on a past friendship.",
  },
  {
    "album": "evermore",
    "name": "marjorie",
    "description": " is about her grandmother.",
  },
  {
    "album": "evermore",
    "name": "evermore",
    "description": " talks about the pain of a failed friendship.",
  },
  {
    "album": "evermore",
    "name": "it's time to go",
    "description": " is about listening to your instincts.",
  },
  {
    "album": "Midnights",
    "name": "Anti Hero",
    "description": " is about being your own enemy.",
  },
  {
    "album": "Midnights",
    "name": "You're On Your Own Kid",
    "description": " is about a lot of things. Figuring those out is an exercise for the reader.",
  },
  {
    "album": "Midnights",
    "name": "Vigilante Shit",
    "description": " is about empowerment, and self confidence.",
  },
  {
    "album": "Midnights",
    "name": "Karma",
    "description": " is about karma.",
  },
  {
    "album": "Midnights",
    "name": "Bigger Than The Whole Sky",
    "description": " is about a miscarriage.",
  },
  {
    "album": "Midnights",
    "name": "Dear Reader",
    "description": " is her giving some advice, and is very introspective.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "Florida!!!",
    "description": " is about escaping and reinventing oneself.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "Who's Afraid of Little Old Me?",
    "description": " is about being judged.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "I Can Do It With a Broken Heart",
    "description": " is about doing your job and appearing ok, no matter how you feel.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "Clara Bow",
    "description": " is about women in the entertainment industry.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "The Albatross",
    "description": " is open to interpretation.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "I Hate It Here",
    "description": " is about hating the world/reality.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "thanK you aIMee",
    "description": " proves that being bullied builds character.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "Cassandra",
    "description": " is about betrayal and truth telling.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "The Bolter",
    "description": " is about self-preservation and escaping.",
  },
  {
    "album": "THE TORTURED POETS DEPARTMENT",
    "name": "Robin",
    "description": " describes a child's world.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Elizabeth Taylor",
    "description": " is about being in the spotlight.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Father Figure",
    "description": " is about power, control, and big dick energy.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Eldest Daughter",
    "description": " is about being an eldest daughter.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Eldest Daughter",
    "description": " is about being an eldest daughter.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Actually Romantic",
    "description": " is actually about a one-sided adversarial relationship.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "Wi$h Li$t",
    "description": " is about fame, privacy, and the wish to have a simple life.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "CANCELLED!",
    "description": " is about cancel culture.",
  },
  {
    "album": "The Life of a Showgirl",
    "name": "The Life of a Showgirl",
    "description": " is about selling one's soul to get a chance to be in the spotlight.",
  },
];


export default function NotRomanticPage() {
  return (
    <Box m={2}>
      <Typography variant="h4">
        Taylor Swift songs not about romance/relationships
      </Typography>

      <Box m={2}></Box>

      <Typography>
        I made this page because people kept asking me:
        "Does Taylor Swift have any songs not about love, relationships, or heartbreak?"
        There are {NOT_ROMANTIC_SONGS_DATA.length} songs on this page.
      </Typography>

      <Box m={2}></Box>

      {ALBUM_ORDER.map(album => {
        const album_songs = NOT_ROMANTIC_SONGS_DATA.filter(d => d.album === album);

        if (album_songs.length === 0) {
          return null;
        }

        return (
          <>
            <Typography key={album} variant="h5">
              <Box
                component="img"
                sx={{
                  height: 30,
                  width: 30,
                }}
                alt="Album Img"
                src={ALBUM_LOGOS[album]}
                mr={1}
              />
              {album}
            </Typography>
            {album_songs.map(song => {
              const href = generateSongHref(album, song.name);

              return (
                <Typography key={song.name}>
                  <Link href={href}>{song.name}</Link> {song.description}
                </Typography>
              );
            })}
          </>
        );
      })}
    </Box>
  );
}