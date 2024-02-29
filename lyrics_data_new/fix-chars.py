import os
from os.path import join
from os import path

def replaceChars(filepath):
    lyrics = ""
    with open(filepath, 'r') as f:
        lyrics = f.read()

        lyrics = lyrics.replace("е", "e")
        lyrics = lyrics.replace("”", "\"")
        lyrics = lyrics.replace("“", "\"")
        lyrics = lyrics.replace("’", "\'")
        lyrics = lyrics.replace("\u2005", " ")
        lyrics = lyrics.replace("\u205f", " ")
        lyrics = lyrics.replace("\u200b", " ")
        lyrics = lyrics.replace("—", "-") # this is a long dash
        lyrics = lyrics.replace("–", "-") # this is a different long dash
        lyrics = lyrics.replace("…", "...")

    with open(filepath, 'w') as f:
        f.write(lyrics)


for path in ["05_1989/17_slut.txt", "05_1989/18_say-dont-go.txt", "05_1989/19_now-that-we-dont-talk.txt", "05_1989/20_suburban-legends.txt", "05_1989/21_is-it-over-now.txt"]:
  replaceChars(path)
