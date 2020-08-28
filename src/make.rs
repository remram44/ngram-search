use lookup::Ngrams;

fn main() {
    let mut builder = Ngrams::builder();
    builder.add("lb", 1);

    // Serialize
    let mut output = std::fs::File::create("trie.db").unwrap();
    builder.write(&mut output).unwrap();
}
