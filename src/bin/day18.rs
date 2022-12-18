#![allow(warnings, unused)]

use anyhow::anyhow;
use anyhow::{bail, Result};
use core::fmt;
use itertools::Itertools;
use sscanf::regex::Regex;
use std::cmp::{max, min};
use std::collections::{vec_deque, HashSet, VecDeque};
use std::ops::{Range, RangeInclusive};
use std::{collections::HashMap, error::Error};
use utils::get_lines;

type Pos = (i32, i32, i32);
type Cubes = HashSet<Pos>;

fn parse_cubes(lines: &Vec<String>) -> Result<Cubes> {
    let mut cubes = Cubes::new();

    for l in lines {
        let it = l.split(',');
        let numbers: Vec<i32> = it.map(|x| x.parse::<i32>().unwrap()).collect();
        let (x, y, z) = (numbers[0], numbers[1], numbers[2]);
        println!("Inserting {x}, {y}, {z}");
        cubes.insert((x, y, z));
    }

    Ok(cubes)
}

fn add_pos(a: &Pos, b: &Pos) -> Pos {
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
}

fn check_cubes(cubes: Cubes) -> i32 {
    let mut total_surface_area = 0;

    let directions = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let cubes: HashSet<Pos> = HashSet::from_iter(cubes.iter().cloned());

    // Flood from the outside
    let max_x = cubes.iter().map(|x| x.0).max().unwrap() + 1;
    let min_x = cubes.iter().map(|x| x.0).min().unwrap() - 1;

    let max_y = cubes.iter().map(|x| x.1).max().unwrap() + 1;
    let min_y = cubes.iter().map(|x| x.1).min().unwrap() - 1;

    let max_z = cubes.iter().map(|x| x.2).max().unwrap() + 1;
    let min_z = cubes.iter().map(|x| x.2).min().unwrap() - 1;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((min_x, min_y, min_z));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);
        println!("Doing {current:?}");

        let neighbors = directions.iter().map(|x| add_pos(&current, x));
        for n in neighbors {
            if cubes.contains(&n) {
                total_surface_area += 1;
            } else {
                if n.0 >= min_x
                    && n.0 <= max_x
                    && n.1 >= min_y
                    && n.1 <= max_y
                    && n.2 >= min_z
                    && n.2 <= max_z
                {
                    queue.push_back(n);
                }
            }
        }
    }

    total_surface_area
}

fn main() -> Result<()> {
    let input = get_lines("day18.txt")?;
    let cubes = parse_cubes(&input)?;
    let surface_area = check_cubes(cubes);
    println!("{surface_area}");
    Ok(())
}
