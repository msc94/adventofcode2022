use std::collections::HashSet;

fn parse_backpack(line: &String) -> (String, String) {
    let length = line.chars().count();
    assert!(length % 2 == 0);

    let c1 = line.chars().take(length / 2).collect();
    let c2 = line.chars().skip(length / 2).take(length / 2).collect();

    (c1, c2)
}

fn get_shared_item(c1: &String, c2: &String) -> Option<char> {
    let mut characters = HashSet::new();

    for c in c1.chars() {
        characters.insert(c);
    }

    for c in c2.chars() {
        if characters.contains(&c) {
            return Some(c);
        }
    }

    None
}

fn get_shared_group_item(b1: &String, b2: &String, b3: &String) -> Option<char> {
    let b1s = HashSet::<_>::from_iter(b1.chars());
    let b2s = HashSet::<_>::from_iter(b2.chars());
    let b3s = HashSet::<_>::from_iter(b3.chars());

    for b1c in b1s {
        if b2s.contains(&b1c) && b3s.contains(&b1c) {
            return Some(b1c);
        }
    }

    None
}

fn get_priority(c: char) -> Option<u32> {
    let cp = c as u32;

    if cp >= 'a' as u32 && cp <= 'z' as u32
    {
        Some((cp - 'a' as u32) + 1)
    }
    else if cp >= 'A' as u32 && cp <= 'Z' as u32
    {
        Some((cp - 'A' as u32) + 27)
    }
    else
    {
        None
    }
}

fn main() {
    let lines = utils::get_lines("day3.txt").unwrap();

    let mut sum = 0;

    for chunk in lines.chunks_exact(3) {
        let b1 = &chunk[0];
        let b2 = &chunk[1];
        let b3 = &chunk[2];
        let shared = get_shared_group_item(b1, b2, b3).unwrap();
        let priority = get_priority(shared).unwrap();
        sum += priority;
    }

    println!("Sum {}", sum);
}
