use std::env;
use std::path::Path;

use lookup::Ngrams;

fn search(data: &mut Ngrams, string: &str) {
    println!("\nSearching {}", string);

    let matches = data.search(string).unwrap();

    // Print results
    println!("Final results ({}):", matches.len());
    for (id, score) in matches.iter().take(10) {
        println!("  id: {}, score: {:.3}", id, score);
    }
}

fn main() {
    let mut data = Ngrams::open(Path::new("trie.db")).unwrap();

    let mut args = env::args();
    let _arg0 = args.next();
    for arg in args {
        search(&mut data, &arg);
    }
}
