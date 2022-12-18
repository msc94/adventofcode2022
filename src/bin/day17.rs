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

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

fn parse_pattern(pattern: &str) -> Result<Vec<Direction>> {
    pattern
        .chars()
        .map(|x| match x {
            '<' => Ok(Direction::LEFT),
            '>' => Ok(Direction::RIGHT),
            _ => Err(anyhow!("Wrong char")),
        })
        .collect()
}

type Position = (i32, i32);

enum RockShape {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
}

#[derive(Clone)]
struct Map {
    occupied: HashSet<Position>,
}

impl Map {
    fn is_occupied(&self, pos: Position) -> bool {
        if pos.0 <= 0 || pos.0 >= 8 {
            return true;
        }

        if pos.1 <= 0 {
            return true;
        }

        self.occupied.contains(&pos)
    }

    fn height(&self) -> i32 {
        self.occupied
            .iter()
            .map(|x| x.0)
            .max()
            .unwrap_or(0)
    }

    fn print(&self) {
        for y in (0..=self.height()) {
            for x in (0..8) {
                let c = match self.is_occupied((x, y)) {
                    true => '#',
                    false => '.',
                };
                print!("{c}");
            }
            println!("");
        }
    }
}

struct Game {
    map: Map,
    pattern: Vec<Direction>,
}

impl Game {
    fn new(pattern: Vec<Direction>) -> Game {
        Game {
            map: Map { occupied: HashSet::new() },
            pattern,
        }
    }

    fn turn() {
    }

    fn run() {

    }
}

fn run_on(pattern: &str) -> Result<i32> {
    let pattern = parse_pattern(pattern)?;
    Ok(0)
}

fn main() -> Result<()> {
    let example_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let result = run_on(example_pattern)?;
    println!("{}", result);
    Ok(())
}
