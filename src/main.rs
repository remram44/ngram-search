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

fn main() {
    let mut data: Vec<Entry> = Vec::new();
    for (trigraph, count, total_ngrams) in &[
        ("$$l", 1, 4),
        ("$lb", 1, 4),
        ("lb$", 1, 4),
        ("b$$", 1, 4),
    ] {
        let mut pos = &mut data;
        for character in trigraph.chars() {
            let character = character as u32;
            let mut idx = None;
            for (i, e) in pos.iter_mut().enumerate() {
                let e = match e {
                    Entry::Leaf(_) => panic!("Found Leaf instead of Branch"),
                    Entry::Branch(b) => b,
                };
                if e.character == character {
                    // We found the right branch, go down

                    // We can't assign to `pos` here, doesn't pass borrow checker
                    // So we store the index in idx and assign pos below
                    idx = Some(i);
                    break;
                }
            }

            // If we didn't find an entry, add it
            let idx = if let Some(idx) = idx {
                idx
            } else {
                pos.push(Entry::Branch(Branch {
                    character,
                    entries: vec![],
                }));
                pos.len() - 1
            };

            // Change the reference to that new entry
            let e = if let Entry::Branch(b) = &mut pos[idx] { b } else { panic!() };
            pos = &mut e.entries;
        }

        // Now insert the leaf
        pos.push(Entry::Leaf(Leaf {
            id: 1,
            count: *count,
            total_ngrams: *total_ngrams,
        }));
    }
}
