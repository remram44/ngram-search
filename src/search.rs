use byteorder::{self, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Read};

struct Leaf {
    id: u32,
    count: u8,
    total_ngrams: u8,
}

type Order = byteorder::BigEndian;

fn search<R: Read + Seek>(data: &mut R, trigram: &str) -> std::io::Result<Vec<Leaf>> {
    data.seek(SeekFrom::Start(0))?;
    for character in trigram.chars() {
        let character = character as u32;

        // Check that this is a branch
        if data.read_u8()? != 1 {
            panic!("Invalid branch");
        }

        // Look for the character we need
        let size = data.read_u32::<Order>()?;
        let mut found = None;
        for _ in 0..size {
            let c = data.read_u32::<Order>()?;
            let p = data.read_u32::<Order>()?;
            if c == character {
                found = Some(p);
                break;
            }
        }

        // Move down
        match found {
            Some(pos) => {
                data.seek(SeekFrom::Start(pos as u64))?;
            }
            None => return Ok(Vec::new()),
        }
    }

    // Read leaves
    if data.read_u8()? != 2 {
        panic!("Invalid leaf record");
    }
    let mut leaves = Vec::new();
    let size = data.read_u32::<Order>()?;
    for _ in 0..size {
        let id = data.read_u32::<Order>()?;
        let count = data.read_u8()?;
        let total_ngrams = data.read_u8()?;
        leaves.push(Leaf {
            id,
            count,
            total_ngrams,
        });
    }

    Ok(leaves)
}

fn main() {
    let trigrams = [
        ("$$l", 1u32),
        ("$lb", 1),
        ("lb$", 1),
        ("b$$", 1),
    ];
    let total_ngrams: u32 = trigrams.iter().map(|(_, c)| c).sum();
    let mut data = std::fs::File::open("trie.db").unwrap();

    // Look for each trigram in turn
    let mut matches: HashMap<u32, (u32, u8)> = HashMap::new(); // id -> (nb_shared_ngrams, total_ngrams)
    for (trigram, count) in &trigrams {
        let leaves = search(&mut data, trigram).unwrap();

        // Print
        println!("{}: {} hits:", trigram, leaves.len());
        for leaf in &leaves {
            println!(
                "  {{ id: {}, count: {}, total_ngrams: {} }}",
                leaf.id, leaf.count, leaf.total_ngrams,
            );
        }

        // Update results
        for leaf in &leaves {
            let match_ = matches.entry(leaf.id).or_insert((0, leaf.total_ngrams));
            match_.0 += (*count).min(leaf.count as u32);
        }
    }

    // Sort results
    let mut matches = matches.drain().map(|(id, (shared, ngrams))| {
        let allgrams = total_ngrams + ngrams as u32 - shared;
        let score = shared as f32 / allgrams as f32;
        (id, score)
    }).collect::<Vec<_>>();
    matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print results
    println!("Final results ({}):", matches.len());
    for (id, score) in matches {
        println!("  id: {}, score: {:.3}", id, score);
    }
}
