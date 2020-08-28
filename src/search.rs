use byteorder::{self, ReadBytesExt};
use std::io::{Seek, SeekFrom, Read};

struct Leaf {
    id: u32,
    count: u8,
    total_ngrams: u8,
}

type Order = byteorder::BigEndian;

fn search<R: Read + Seek>(data: &mut R, trigram: &str) -> std::io::Result<Vec<Leaf>> {
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
    let trigram = "lb$";
    let mut data = std::fs::File::open("trie.db").unwrap();
    let leaves = search(&mut data, trigram).unwrap();
    println!("{} results:", leaves.len());
    for leaf in leaves {
        println!(
            "  {{ id: {}, count: {}, total_ngrams: {} }}",
            leaf.id, leaf.count, leaf.total_ngrams,
        );
    }
}
