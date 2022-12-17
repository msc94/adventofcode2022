#![allow(warnings, unused)]

use anyhow::anyhow;
use anyhow::{bail, Result};
use itertools::Itertools;
use sscanf::regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: i32,
    connections: Vec<String>,
}

fn parse_input(lines: Vec<String>) -> Result<HashMap<String, Valve>> {
    let mut valves = HashMap::new();

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let name = parts[1];
        let flow_rate = parts[4]
            .split(['=', ';'])
            .nth(1)
            .ok_or(anyhow!("Error"))?
            .parse::<i32>()?;
        let connections = parts[9..]
            .iter()
            .map(|x| x.replace(',', ""))
            .collect::<Vec<String>>();

        let valve = Valve {
            flow_rate,
            connections,
        };
        valves.insert(name.to_string(), valve);
    }

    return Ok(valves);
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct SolverContext {
    location: String,
    time_left: i32,
    open_valves: Vec<String>,
}

struct Solver {
    valves: HashMap<String, Valve>,
    solutions: HashMap<SolverContext, i32>,
}

impl Solver {
    fn solve(&mut self, context: SolverContext) -> Result<i32> {
        if self.solutions.contains_key(&context) {
            return Ok(self.solutions[&context]);
        }

        let time_left = context.time_left;

        if time_left <= 0 {
            return Ok(0);
        }

        let current_position = &context.location;
        let current_valve = self.valves[current_position].clone();
        let current_open = context.open_valves.contains(current_position);
        
        let mut possibilities: Vec<i32> = Vec::new();

        for connection in current_valve.connections {
            // We can either open the valve...
            if !current_open && current_valve.flow_rate > 0 {
                let mut open_context = context.clone();
                open_context.location = connection.clone();
                open_context.open_valves.push(context.location.clone());
                open_context.time_left -= 2;
                let opened =
                    (time_left - 1) * current_valve.flow_rate + self.solve(open_context)?;
                possibilities.push(opened);
            }

            // ... or leave it closed and go on
            let mut closed_context = context.clone();
            closed_context.location = connection;
            closed_context.time_left -= 1;
            let closed = self.solve(closed_context)?;
            possibilities.push(closed);
        }

        let solution = possibilities
            .iter()
            .max()
            .ok_or(anyhow!("Can't get max"))
            .copied()?;

        self.solutions.insert(context, solution);
        Ok(solution)
    }
}

fn run_on_file(file: &str) -> Result<i32> {
    let lines = utils::get_lines(file)?;
    let valves = parse_input(lines)?;

    let mut solver = Solver {
        valves,
        solutions: HashMap::new(),
    };
    let solution = solver.solve(SolverContext {
        location: "AA".to_string(),
        time_left: 30,
        open_valves: Vec::new(),
    });
    solution
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = run_on_file("day15.txt")?;
    println!("{}", result);
    Ok(())
}
