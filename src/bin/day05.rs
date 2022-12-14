use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};
use itertools::Itertools;

fn parse_input(lines: &Vec<String>) -> (HashMap<i32, VecDeque<char>>, Vec<(i32, i32, i32)>) {
    let mut crates: HashMap<i32, VecDeque<_>> = HashMap::new();
    let mut moves: Vec<(i32, i32, i32)> = Vec::new();

    for l in lines {
        if l.contains('[') {
            let chars: Vec<char> = l.chars().collect();
            for (i, chunk) in chars.chunks(4).enumerate() {
                let letter = chunk[1];
                if letter != ' ' {
                    let index = (i as i32) + 1;
                    let entry = crates.entry(index).or_default();
                    entry.push_front(letter);
                }
            }
        } else if l.starts_with("move") {
            let parsed = sscanf::sscanf!(l, "move {} from {} to {}", i32, i32, i32);
            moves.push(parsed.unwrap());
        }
    }

    return (crates, moves);
}

fn move_crate(crates: &mut HashMap<i32, VecDeque<char>>, from: i32, to: i32) {
    let from_crates = crates.get_mut(&from).unwrap();
    let c = from_crates.pop_back().unwrap();
    println!("Moving crate {} from {} to {}", c, from, to);

    let to_crates = crates.get_mut(&to).unwrap();
    to_crates.push_back(c);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = utils::get_lines("day5.txt")?;
    let (mut crates, moves) = parse_input(&lines);

    println!("{:?}", crates);
    println!("{:?}", moves);

    for (amount, from, to) in moves {
        for _ in 0..amount {
            println!("{:?}", crates);
            move_crate(&mut crates, from, to);
        }
    }

    for key in crates.keys().sorted() {
        let cr = &crates[key];
        let last = cr.iter().last().unwrap();
        print!("{}", last);
    }

    println!();

    Ok(())
}
