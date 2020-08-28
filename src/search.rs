use std::path::Path;

use lookup::Ngrams;

fn search(data: &mut Ngrams, string: &str) {
    println!("\nSearching {}", string);

    let matches = data.search(string).unwrap();

    // Print results
    println!("Final results ({}):", matches.len());
    for (id, score) in matches {
        println!("  id: {}, score: {:.3}", id, score);
    }
}

fn main() {
    let mut data = Ngrams::open(Path::new("trie.db")).unwrap();

    search(&mut data, "mam");
    search(&mut data, "ham");
    search(&mut data, "pam");
}
