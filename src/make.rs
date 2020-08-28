use byteorder::{self, WriteBytesExt};
use std::io::{Seek, SeekFrom, Write};

struct Branch {
    entries: Vec<Entry>,
    character: u32,
}

struct Leaf {
    id: u32,
    count: u8,
    total_ngrams: u8,
}

enum Entry {
    Branch(Branch),
    Leaf(Leaf),
}

fn add(mut data: &mut Vec<Entry>, trigram: &str, id: u32, count: u8, total_ngrams: u8) {
    for character in trigram.chars() {
        let character = character as u32;
        let mut idx = None;
        for (i, e) in data.iter_mut().enumerate() {
            let e = match e {
                Entry::Leaf(_) => panic!("Found Leaf instead of Branch"),
                Entry::Branch(b) => b,
            };
            if e.character == character {
                // We found the right branch, go down

                // We can't assign to `data` here, doesn't pass borrow checker
                // So we store the index in idx and assign data below
                idx = Some(i);
                break;
            }
        }

        // If we didn't find an entry, add it
        let idx = if let Some(idx) = idx {
            idx
        } else {
            data.push(Entry::Branch(Branch {
                character,
                entries: vec![],
            }));
            data.len() - 1
        };

        // Change the reference to that new entry
        let e = if let Entry::Branch(b) = &mut data[idx] { b } else { panic!() };
        data = &mut e.entries;
    }

    // Now insert the leaf
    data.push(Entry::Leaf(Leaf {
        id,
        count,
        total_ngrams,
    }));
}

type Order = byteorder::BigEndian;

fn write_branch<W: Write + Seek>(entries: &[Entry], output: &mut W) -> std::io::Result<u64> {
    // Seek to end of stream, save position
    let pos = output.seek(SeekFrom::End(0))?;

    let is_branch = match entries.first() {
        None => panic!("Empty entry"),
        Some(Entry::Branch(_)) => true,
        Some(Entry::Leaf(_)) => false,
    };

    // Tag content
    output.write_all(&[if is_branch { 1u8 } else { 2u8 }])?;

    // Write length
    output.write_u32::<Order>(entries.len() as u32)?;

    let start = pos + 1 + 4;

    if is_branch {
        // Reserve space for our record
        let mut data = Vec::new();
        data.resize((4 + 4) * entries.len(), 0);
        output.write_all(&data)?;

        // Recursively write the entries at the end of the stream, each time
        // updating the entry in our record
        for (i, entry) in entries.iter().enumerate() {
            match entry {
                Entry::Branch(Branch {
                    entries: branch_entries,
                    character,
                }) => {
                    // Recursively write at the end
                    let branch_pos = write_branch(branch_entries, output)?;

                    // Update the entry in our record to point there
                    output.seek(SeekFrom::Start(start + (4 + 4) * (i as u64)))?;
                    output.write_u32::<Order>(*character)?;
                    output.write_u32::<Order>(branch_pos as u32)?;
                }
                Entry::Leaf(_) => panic!("Leaf in a branch"),
            }
        }
    } else {
        // Write the leaves
        for entry in entries {
            match entry {
                Entry::Leaf(Leaf {
                    id,
                    count,
                    total_ngrams,
                }) => {
                    output.write_u32::<Order>(*id)?;
                    output.write_all(&[*count, *total_ngrams])?;
                }
                Entry::Branch(_) => panic!("Branch in a leaf"),
            }
        }
    }

    Ok(pos)
}

fn main() {
    let mut data: Vec<Entry> = Vec::new();
    for (trigram, count, total_ngrams) in &[
        ("$$l", 1, 4),
        ("$lb", 1, 4),
        ("lb$", 1, 4),
        ("b$$", 1, 4),
    ] {
        add(&mut data, trigram, 1, *count, *total_ngrams);
    }

    // Serialize
    let mut output = std::fs::File::create("trie.db").unwrap();
    write_branch(&data, &mut output).unwrap();
}
