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
}
