use lookup::Ngrams;

fn main() {
    let mut builder = Ngrams::builder();
    for (trigram, count, total_ngrams) in &[
        ("$$l", 1, 4),
        ("$lb", 1, 4),
        ("lb$", 1, 4),
        ("b$$", 1, 4),
    ] {
        builder.add_trigram(trigram, 1, *count, *total_ngrams);
    }

    // Serialize
    let mut output = std::fs::File::create("trie.db").unwrap();
    builder.write(&mut output).unwrap();
}
