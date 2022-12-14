fn get_play(c1: char, c2: char) -> Option<char> {
    match c2 {
        // Win
        'Z' => match c1 {
            'A' => Some('B'),
            'B' => Some('C'),
            'C' => Some('A'),
            _ => None
        },
        // Draw
        'Y' => Some(c1),
        // Loss
        'X' => match c1 {
            'A' => Some('C'),
            'B' => Some('A'),
            'C' => Some('B'),
            _ => None
        },
        _ => None
    }
}

fn get_chosen_score(c: char) -> Option<i32> {
    match c {
        // Rock
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        _ => None,
    }
}

fn get_play_score(a: char, b: char) -> Option<i32> {
    match (a, b) {
        ('A', 'A') => Some(3),
        ('A', 'B') => Some(6),
        ('A', 'C') => Some(0),
        ('B', 'A') => Some(0),
        ('B', 'B') => Some(3),
        ('B', 'C') => Some(6),
        ('C', 'A') => Some(6),
        ('C', 'B') => Some(0),
        ('C', 'C') => Some(3),
        _ => None,
    }
}

fn main() {
    let lines = utils::get_lines("day2.txt").unwrap();

    let mut score = 0;

    for l in &lines {
        let mut parts = l.split(' ');

        let c1 = parts.next().unwrap().chars().next().unwrap();
        let c2 = parts.next().unwrap().chars().next().unwrap();
        let play = get_play(c1, c2).unwrap();

        println!("c1: {}, c2: {}, play: {}", c1, c2, play);
        println!("chosen: {}", get_chosen_score(play).unwrap_or(0));
        println!("play: {}", get_play_score(c1, play).unwrap_or(0));

        score += get_chosen_score(play).unwrap_or(0);
        score += get_play_score(c1, play).unwrap_or(0);
    }

    println!("Score: {}", score);
}
