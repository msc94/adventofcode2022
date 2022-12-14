#![allow(warnings, unused)]

use anyhow::anyhow;
use anyhow::{bail, Result};
use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::{Range, RangeInclusive};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
enum CellType {
    Stone,
    Sand,
}

fn parse_point(string: &str) -> Result<(i32, i32)> {
    let results = string
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    Ok((results[0], results[1]))
}

fn range_from_values(a: i32, b: i32) -> RangeInclusive<i32> {
    return min(a, b)..=max(a, b);
}

fn build_map(input: Vec<String>) -> Result<HashMap<(i32, i32), CellType>> {
    let mut result = HashMap::new();

    for line in input {
        let points: Vec<&str> = line.split(" -> ").collect();
        for i in 0..points.len() {
            if i + 1 >= points.len() {
                break;
            }

            let first = parse_point(points[i])?;
            let second = parse_point(points[i + 1])?;

            for x in range_from_values(first.0, second.0) {
                for y in range_from_values(first.1, second.1) {
                    result.insert((x, y), CellType::Stone);
                }
            }
        }
    }

    Ok(result)
}

fn get_tile(field: &HashMap<(i32, i32), CellType>, pos: &(i32, i32), y_boundary: i32) -> Option<CellType> {
    if pos.1 == y_boundary + 2 {
        return Some(CellType::Stone);
    }

    field.get(pos).cloned()
}

fn simulate(mut field: HashMap<(i32, i32), CellType>) -> Result<i32> {
    let original_size = field.len();

    let y_boundary = field
        .keys()
        .map(|x| x.1)
        .max()
        .ok_or(anyhow!("No boundary?"))?;

    let mut current_sand = (500, 0);
    while let None = field.get(&(500, 0)) {
        match (
            get_tile(&field, &(current_sand.0, current_sand.1 + 1), y_boundary),
            get_tile(&field, &(current_sand.0 - 1, current_sand.1 + 1), y_boundary),
            get_tile(&field, &(current_sand.0 + 1, current_sand.1 + 1), y_boundary),
        ) {
            (None, _, _) => {
                current_sand.1 += 1;
            },
            (_, None, _) => {
                current_sand.0 -= 1;
                current_sand.1 += 1;
            },
            (_, _, None) => {
                current_sand.0 += 1;
                current_sand.1 += 1;
            },
            (Some(_), Some(_), Some(_)) => {
                // Can't move anywhere
                field.insert(current_sand, CellType::Sand);
                current_sand = (500, 0);
            }
        }
    };

    return Ok((field.len() - original_size) as i32);
}

fn run_on_file(file: &str) -> Result<i32> {
    let lines = utils::get_lines(file)?;
    let field = build_map(lines)?;
    simulate(field)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = run_on_file("day14.txt")?;
    println!("{}", result);
    Ok(())
}
