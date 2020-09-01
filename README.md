This library allows indexing many strings into a file, and then efficiently fuzzy-matching strings against what's been indexed.

Currently, the structure is built in memory before being written to the file, so that phase uses a lot of RAM.

String search is done from the file and requires little memory.

The index is a trie structure in which trigrams can be looked up; results for each trigrams of the input are matched and sorted to get the most similar strings.

Example (Rust):

```rust
// Build index
let mut builder = Ngrams::builder();
builder.add("spam", 0);
builder.add("ham", 1);
builder.add("mam", 2);

// Write it to a file
let mut file = BufWriter::new(File::create(path).unwrap());
builder.write(&mut file).unwrap();

// Search our index
let mut data = Ngrams::open(path).unwrap();
assert_eq!(
    data.search("ham", 0.24).unwrap(),
    vec![
        (1, 1.0), // "ham" is an exact match
        (2, 0.25), // "mam" is close
    ],
);
assert_eq!(
    data.search("spa", 0.2).unwrap(),
    vec![
        (0, 0.22222222), // "spam" is close
    ],
);
```

Example (Python):

```python console
>>> from ngram_search import Ngrams
>>> ngrams = Ngrams(path)
>>> ngrams.search("ham", 0.24)
[(0, 1.0), (2, 0.25)]
>>> ngrams.search("spa", 0.2)
[(0, 0.2222222222222222)]
```
