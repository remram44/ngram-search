This library allows indexing many strings into a file, and then efficiently fuzzy-matching strings against what's been indexed.

Currently, the structure is built from the Rust program in memory before being written to the file, so that phase uses a lot of RAM. You cannot create/change an index from Python.

String search is done from the file and requires little memory.

The index is a trie structure in which trigrams can be looked up; results for each trigrams of the input are matched and sorted to get the most similar strings.

Example::

    >>> from ngram_search import Ngrams
    >>> ngrams = Ngrams(path)
    >>> ngrams.search("ham", 0.24)
    [(0, 1.0), (2, 0.25)]
    >>> ngrams.search("spa", 0.2)
    [(0, 0.2222222222222222)]
