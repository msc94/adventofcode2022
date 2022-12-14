use utils::get_lines;

fn main() {
    let lines = get_lines("day1.txt").unwrap();

    let mut calories = vec![0; 1];

    for l in &lines {
        if l == "" {
            calories.push(0)
        } else {
            let count: i32 = l.parse().unwrap();
            *calories.last_mut().unwrap() += count;
        }
    }

    calories.sort();

    let mut total = 0;
    for _ in 0..3 {
        total += calories.pop().unwrap();
    }

    println!("{}", total);
}
