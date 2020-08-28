use std::io::BufWriter;

use lookup::Ngrams;

fn main() {
    let mut builder = Ngrams::builder();
    builder.add("spam", 1);
    builder.add("ham", 2);
    builder.add("hammock", 3);

    // Serialize
    let mut output = BufWriter::new(File::create("trie.db").unwrap());
    builder.write(&mut output).unwrap();
}
