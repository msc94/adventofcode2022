use std::error::Error;

#[derive(Debug)]
enum Command {
    Noop,
    Add(i32),
}

fn parse_input(lines: Vec<String>) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut result: Vec<Command> = Vec::new();

    for l in lines {
        if l == "noop" {
            result.push(Command::Noop);
        } else if l.starts_with("addx") {
            let mut parts = l.split(" ");
            let num: i32 = parts.nth(1).ok_or("")?.parse()?;
            result.push(Command::Add(num));
        }
    }

    return Ok(result);
}

fn increment_cycle(reg: &i32, cycle: &mut i32, sum: &mut i32) {
    if *cycle == 20 || (*cycle - 20) % 40 == 0 {
        let add = *reg * *cycle;
        *sum += add;
        println!("Cycle: {}, Reg: {}, Add: {}, Sum: {}", *cycle, *reg, add, *sum);
    }
    *cycle += 1;
}

fn execute_commands(commands: Vec<Command>, reg: &mut i32, cycle: &mut i32) {
    let mut sum: i32 = 0;
    for c in commands {
        match c {
            Command::Noop => {
                println!("Noop");
                increment_cycle(reg, cycle, &mut sum);
            }
            Command::Add(x) => {
                println!("Add {}", x);
                increment_cycle(reg, cycle, &mut sum);
                *reg += x;
                increment_cycle(reg, cycle, &mut sum);
            }
        }
    }
    println!("Final sum {}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = utils::get_lines("day10_example.txt")?;
    let input = parse_input(lines)?;

    let mut reg = 1;
    let mut cycle = 1;
    execute_commands(input, &mut reg, &mut cycle);

    Ok(())
}
