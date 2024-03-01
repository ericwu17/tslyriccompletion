# Data Migration

I will be migrating a bunch of lyrics by hand.

Recall that for each data file, the first line is the album,
and the next line is the song name. Each subsequent line
is a line of lyrics from the song, except for lines which
begin with a `[`, which are lines marking different
parts of the song (such as verse, chorus, etc). We ignore the marker lines
when extracting a list of lines from which to create guesses,
but the marker lines are included in the raw lyric data (used to display full lyrics).

Below are the changes:

One thing: for better consistency, we will change all -in' endings to -ing.
So gettin' -> getting, standin' -> standing, etc.

The position of line breaks may be changed, to alleviate some confusion. In the old lyrics,
for example, in "You all over me", the game thought that "I lived, I learned" has a unique
successor, but it really did not have a unique successor.

Each line's status of whether it is exclamatory
will now be statically marked. In each line of each song data file, 
if there is at least one `$` character,
then all the text coming after the last `$` are markers, and all the text
coming before the last `$` is the data of the line.
The marker strings for these three statuses are `exclamatory`.

Furthermore, I will introduce a new marker string called `misc_bad` which
will be used to manually exclude a line from being picked as a prompt,
for miscellaneous reasons. 
This marker may contain an optional note explaining why the line was marked as bad.

For every song, the last line of its lyrics should be automatically marked as `misc_bad`,
with an explanation saying that the line doesn't work as a prompt because there is nothing
following it.

The static marking of each lines status allows more flexibility and control in what
lines are used as guesses.

# Changes to the game

Previously, if a line had multiple successors, then it would have never been shown as a
prompt. However, this seems a little bit too restrictive. Now, such a line may still be used
as a prompt, and based on whatever the player has guessed, the game server shall determine
what the "actual" correct answer is, by picking whatever next line would have given the player
a higher score. In other words, the game will try to figure out the position in the song 
where the player is completing the lyric.

When a song has multiple successors, a multiple choice question still has only one
correct choice out of 17. The correct line is randomly chosen.

The criteria for Asking For more status should be improved, to reduce "false positives".
The player must submit a guess that is more than half the length of the candidate answer,
it should be too far any candidate answers to pass.



