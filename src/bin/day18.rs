#![allow(warnings, unused)]

use anyhow::anyhow;
use anyhow::{bail, Result};
use core::fmt;
use itertools::Itertools;
use sscanf::regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
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
    let mut added = Cubes::new();
    let mut surrounded = Cubes::new();
    let mut total_surface_area = 0;
    let mut hidden_surface_area = 0;

    let directions = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for c in cubes {
        println!("Adding {:?}", c);

        total_surface_area += 6;

        let neighbor_positions = directions.iter().map(|x| add_pos(&c, x));
        for n in neighbor_positions {
            if added.contains(&n) {
                hidden_surface_area += 2;
            } else {
                // Check if air tile is surrounded
                let is_surrounded = directions
                    .iter()
                    .map(|x| add_pos(&n, x))
                    .all(|x| x == c || added.contains(&x));

                if is_surrounded {
                    surrounded.insert(n);
                }
            }
        }

        added.insert(c);
    }

    return total_surface_area - hidden_surface_area - 6 * surrounded.len() as i32;
}

fn main() -> Result<()> {
    let input = get_lines("day18.txt")?;
    let cubes = parse_cubes(&input)?;
    let surface_area = check_cubes(cubes);
    println!("{surface_area}");
    Ok(())
}
