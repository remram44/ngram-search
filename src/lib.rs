use byteorder::{self, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::path::Path;

type Order = byteorder::BigEndian;

pub struct Leaf {
    pub id: u32,
    pub count: u8,
    pub total_ngrams: u8,
}

pub struct Branch {
    pub entries: Vec<Entry>,
    pub character: u32,
}

pub enum Entry {
    Branch(Branch),
    Leaf(Leaf),
}

pub struct Ngrams {
    reader: BufReader<File>,
}

impl Ngrams {
    pub fn builder() -> NgramsBuilder {
        Default::default()
    }

    pub fn open(path: &Path) -> std::io::Result<Ngrams> {
        let reader = BufReader::new(File::open(path)?);
        Ok(Ngrams {
            reader,
        })
    }

    pub fn search_ngram(&mut self, trigram: &str) -> std::io::Result<Vec<Leaf>> {
        self.reader.seek(SeekFrom::Start(0))?;
        for character in trigram.chars() {
            let character = character as u32;

            // Check that this is a branch
            if self.reader.read_u8()? != 1 {
                panic!("Invalid branch");
            }

            // Look for the character we need
            let size = self.reader.read_u32::<Order>()?;
            let mut found = None;
            for _ in 0..size {
                let c = self.reader.read_u32::<Order>()?;
                let p = self.reader.read_u32::<Order>()?;
                if c == character {
                    found = Some(p);
                    break;
                }
            }

            // Move down
            match found {
                Some(pos) => {
                    self.reader.seek(SeekFrom::Start(pos as u64))?;
                }
                None => return Ok(Vec::new()),
            }
        }

        // Read leaves
        if self.reader.read_u8()? != 2 {
            panic!("Invalid leaf record");
        }
        let mut leaves = Vec::new();
        let size = self.reader.read_u32::<Order>()?;
        for _ in 0..size {
            let id = self.reader.read_u32::<Order>()?;
            let count = self.reader.read_u8()?;
            let total_ngrams = self.reader.read_u8()?;
            leaves.push(Leaf {
                id,
                count,
                total_ngrams,
            });
        }

        Ok(leaves)
    }

    pub fn search_trigrams(&mut self, trigrams: &[(&str, u32)]) -> std::io::Result<Vec<(u32, f32)>> {
        let total_ngrams: u32 = trigrams.iter().map(|(_, c)| c).sum();

        // Look for each trigram in turn
        let mut matches: HashMap<u32, (u32, u8)> = HashMap::new(); // id -> (nb_shared_ngrams, total_ngrams)
        for (trigram, count) in trigrams {
            let leaves = self.search_ngram(trigram).unwrap();

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

        Ok(matches)
    }
}

#[derive(Default)]
pub struct NgramsBuilder {
    data: Vec<Entry>,
}

impl NgramsBuilder {
    pub fn add_trigram(&mut self, trigram: &str, id: u32, count: u8, total_ngrams: u8) {
        let mut data = &mut self.data;
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

        // Now insert the leaf, sorted by id
        // Find position
        let idx = bisect_leaves(data, id);
        data.insert(
            idx,
            Entry::Leaf(Leaf {
                id,
                count,
                total_ngrams,
            }),
        );
    }

    pub fn write<W: Write + Seek>(&self, output: &mut W) -> std::io::Result<()> {
        write_branch(&self.data, output)?;
        Ok(())
    }
}

fn bisect_leaves(data: &[Entry], id: u32) -> usize {
    let mut low = 0;
    let mut high = data.len();
    while low < high {
        let mid = (low + high) / 2;
        let x = match &data[mid] {
            Entry::Branch(_) => panic!("Branch in the leaves"),
            Entry::Leaf(leaf) => leaf.id,
        };
        if id < x {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

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
