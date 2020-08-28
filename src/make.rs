use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

use lookup::Ngrams;

fn main() {
    let total_lines = BufReader::new(File::open("../test-names.txt").unwrap()).lines().count();
    eprintln!("{} lines total", total_lines);

    // Build from a file
    let input = BufReader::new(File::open("../test-names.txt").unwrap());

    let mut builder = Ngrams::builder();
    for (i, line) in input.lines().enumerate() {
        if i % 500_000 == 0 {
            eprintln!("{} / {}", i, total_lines);
        }
        let line = line.unwrap();
        builder.add(&line, i as u32 + 1);
    }

    // Serialize
    let mut output = BufWriter::new(File::create("trie.db").unwrap());
    builder.write(&mut output).unwrap();
}
