use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

type Trees = Vec<Vec<u32>>;

fn parse_input(lines: Vec<String>) -> Option<Trees> {
    let mut trees: Trees = Vec::new();

    for line in lines {
        let mut line_trees: Vec<u32> = Vec::new();
        for char in line.chars() {
            line_trees.push(char.to_digit(10)?);
        }
        trees.push(line_trees);
    }

    return Some(trees);
}

fn get_tree_at(trees: &Trees, x: u32, y: u32) -> Option<u32> {
    return trees.get(y as usize)?.get(x as usize).copied();
}

fn tree_visible(trees: &Trees, x: u32, y: u32, width: u32, height: u32) -> bool {
    let cur_height = get_tree_at(&trees, x, y);
    println!("Checking {}, {} with height {}", x, y, cur_height.unwrap());

    // left
    for x_ in 0..x {
        if get_tree_at(&trees, x_, y) > cur_height {
            return false;
        }
    }

    // right
    for x_ in x+1..width {
        if get_tree_at(&trees, x_, y) > cur_height {
            return false;
        }
    }

    // up
    for y_ in 0..y {
        if get_tree_at(&trees, x, y_) > cur_height {
            return false;
        }
    }

    // down
    for y_ in y+1..height {
        if get_tree_at(&trees, x, y_) > cur_height {
            return false;
        }
    }

    println!("Visible");
    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = utils::get_lines("day8_example.txt")?;
    let trees = parse_input(lines).ok_or("")?;

    println!("{:?}", trees);

    let height = trees.len();
    let width = trees[0].len();

    let mut visible = 0;
    for y in 0..height {
        for x in 0..width {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                visible += 1;
                continue;
            }

            if tree_visible(&trees, x as u32, y as u32, width as u32, height as u32) {
                visible += 1;
            }
        }
    }

    println!("Visible: {}", visible);

    Ok(())
}
