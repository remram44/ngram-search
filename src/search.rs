use std::env;
use std::path::Path;
use std::time::Instant;

use ngram_search::Ngrams;

fn search(data: &mut Ngrams, string: &str) {
    let start = Instant::now();

    println!("\nSearching {}", string);
    let matches = data.search(string, 0.3).unwrap();
    println!("{:.3} seconds", start.elapsed().as_millis() as f32 / 1000.0);

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
