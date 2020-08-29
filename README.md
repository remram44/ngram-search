This crate allows indexing many strings into a file, and then efficiently fuzzy-matching strings against what's been indexed.

Currently, the structure is built in memory before being written to the file, so that phase uses a lot of RAM.

String search is done from the file and requires little memory.

The index is a trie structure in which trigrams can be looked up; results for each trigrams of the input are matched and sorted to get the most similar strings.
