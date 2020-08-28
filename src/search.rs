use std::path::Path;

use lookup::Ngrams;

fn main() {
    let mut data = Ngrams::open(Path::new("trie.db")).unwrap();

    let matches = data.search("lb").unwrap();

    // Print results
    println!("Final results ({}):", matches.len());
    for (id, score) in matches {
        println!("  id: {}, score: {:.3}", id, score);
    }
}
