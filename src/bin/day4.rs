use std::collections::HashSet;

type Assignment = (i32, i32);

fn parse_assignment(assignment: &String) -> Assignment {
    let mut iter = assignment.split("-");
    let first: i32 = iter.next().unwrap().parse().unwrap();
    let second: i32 = iter.next().unwrap().parse().unwrap();
    (first, second)
}

fn one_contains_other(a: &Assignment, b: &Assignment) -> bool {
    if a.0 >= b.0 && a.1 <= b.1 {
        return true;
    }

    if b.0 >= a.0 && b.1 <= a.1 {
        return true;
    }

    false
}

fn overlap(a: &Assignment, b: &Assignment) -> bool {
    let contains = |a: &Assignment, s: i32| -> bool {
        a.0 >= s && a.1 <= s
    };

    for s in a.0..=a.1 {
        if contains(b, s) {
            return true;
        } else {
            println!("{:?} does not contain {}", b, s);
        }
    }

    false
}

fn parse_line(line: &String) -> (String, String) {
    let mut iter = line.split(",");
    (
        iter.next().unwrap().to_string(),
        iter.next().unwrap().to_string(),
    )
}

fn main() {
    let lines = utils::get_lines("day4_example.txt").unwrap();
    let parsed: Vec<(String, String)> = lines.iter().map(parse_line).collect();

    let assignments: Vec<(Assignment, Assignment)> = parsed
        .iter()
        .map(|x| (parse_assignment(&x.0), parse_assignment(&x.1)))
        .collect();

    println!("Assignment: {:?}", assignments);

    let mut pairs_contained = 0;

    for (a1, a2) in assignments {
        if overlap(&a1, &a2) {
            println!("Contains: {:?}, {:?}", a1, a2);
            pairs_contained += 1;
        }
    }

    println!("{}", pairs_contained);
}
