use std::path::Path;

use lookup::Ngrams;

fn main() {
    let trigrams = [
        ("$$l", 1u32),
        ("$lb", 1),
        ("lb$", 1),
        ("b$$", 1),
    ];

    let mut data = Ngrams::open(Path::new("trie.db")).unwrap();

    let matches = data.search_trigrams(&trigrams).unwrap();

    // Print results
    println!("Final results ({}):", matches.len());
    for (id, score) in matches {
        println!("  id: {}, score: {:.3}", id, score);
    }
}
