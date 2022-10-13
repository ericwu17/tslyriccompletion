import json
import os
from os.path import join
from os import path
import nltk
import unicodedata

raw_lyric_dir = "lyrics"
comp_lyric_dir = "lyrics-compiled"
working_dir = path.dirname(__file__)


NON_ASCII = []
def normalize(FILE_IN):
	with open(FILE_IN, 'r') as f:
		lyrics = f.read()
		# for char in lyrics:
		# 	if 0 <= ord(char) <= 127:
		# 		pass #this is an ascii character
		# 	else:
		# 		print(char)
		# 		if char not in NON_ASCII:
		# 			NON_ASCII.append(char)
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

	with open(FILE_IN, 'w') as f:
		f.write(lyrics)


def read_file(FILE_IN):
	words = []
	end_positions = []
	with open(FILE_IN, 'r') as f:
		for currLine, line in enumerate(f.readlines()):
			currLine += 1
			line = line.strip()
			#skip blank lines
			if line == "":
				continue
			#process song section labels:
			if line.startswith("["):
				if not line.endswith("]"):
					print(f"conversion error on {FILE_IN}")
					return f"Missing closing bracket expected on line {currLine}."
				continue

			#check for inappropriate use of dashes: only double dashes are allowed
			# if "-" in line.replace("--", "") or "---" in line:
			# 	return f"Bad use of dashes on line {currLine}."
			
			
			filteredLine = ""
			for char in line:
				if char in {'"', "'", "(", ")", ",", ".", "?", "!"}: #we filter out these characters
					continue
				filteredLine += char
			words += list(filteredLine.split())
			end_positions.append(len(words))
			
	# this function returns a list of words indices at the end of each line, and also returns two strings: one for the song's words
	# and another for the song's words' tags.
	tagged = nltk.pos_tag(words)
	return {"end_positions" : end_positions, "words": " ".join([x[0] for x in tagged]), "tags": " ".join([x[1] for x in tagged])}

os.chdir(working_dir)
os.chdir(raw_lyric_dir)
# album_titles = [name.partition("_")[2] for name in os.listdir(".") if os.path.isdir(name)]
album_titles = [name for name in os.listdir(".") if os.path.isdir(name)]

os.chdir(working_dir)
all_lyrics = {}
song_dirs = {}
for album in album_titles:
	os.chdir(join(working_dir, raw_lyric_dir, album))
	for path in os.listdir("."):
		if not os.path.isfile(path):
			continue
		normalize(path)
		
		album_formatted = album.partition("_")[2]
		name = os.path.splitext(path.partition("_")[2])[0]
		all_lyrics[f"{album_formatted}--{name}"] = read_file(path)
		song_dirs[f"{album_formatted}--{name}"] = os.path.join(raw_lyric_dir, album, path)
os.chdir(working_dir)
os.chdir(comp_lyric_dir)
# for d in all_lyrics.keys():
# 	all_lyrics[d] = [w.lower() for w in all_lyrics[d]]
with open("lyrics.json", "w") as f:
	f.write(json.dumps(all_lyrics , indent = 2))
# with open("raw-lyric-dirs.json", "w") as f:
# 	f.write(json.dumps(song_dirs))


#now we do word-analysis to generate a dictionary to tell us how many songs each word appears in.
#first we convert the lyrics to sets to make lookup faster
all_lyrics = {key: set(value) for (key, value) in all_lyrics.items()} 
all_words = set()
for lyric_set in all_lyrics.values():
	all_words.update({w.lower() for w in lyric_set})

word_frequencies = {}
for word in all_words:
	count = 0
	for lyric_set in all_lyrics.values():
		if word in lyric_set:
			count += 1
	word_frequencies[word] = count

# with open("word-frequencies.json", "w") as f:
# 	f.write(json.dumps(word_frequencies))



print("DONE")
