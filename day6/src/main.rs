use std::fs::File;
use std::io::{BufRead, BufReader};

fn next_step(map: &[Vec<char>], location: (usize, usize)) -> (isize, isize) {
    let (r, c) = location;
    let dr: isize = match map[r][c] {
        '^' => -1,
        'v' => 1,
        _ => 0,
    };

    let dc: isize = match map[r][c] {
        '<' => -1,
        '>' => 1,
        _ => 0,
    };

    let tr = r as isize + dr;
    let tc = c as isize + dc;

    (tr, tc)
}

fn step_guard(map: &mut [Vec<char>], from: (usize, usize), to: (usize, usize)) -> bool {
    let (r, c) = from;
    let (tr, tc) = to;

    let unvisited = map[tr][tc] == '.';
    map[tr][tc] = map[r][c];
    map[r][c] = 'X';

    println!(
        "Guard moved {} to ({}, {}) [{}]",
        map[tr][tc], tr, tc, if unvisited { "new" } else { "has been before" }
    );

    unvisited
}

fn turn_guard_right(map: &mut [Vec<char>], location: (usize, usize)) {
    let (r, c) = location;

    map[r][c] = match map[r][c] {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Guard is not at ({}, {})!", r, c),
    };

    println!("Guard turned right: {} at ({}, {})", map[r][c], r, c);
}

fn within_lab(map: &[Vec<char>], location: (isize, isize)) -> bool {
    let (tr, tc) = location;
    tr >= 0 && tr < map.len() as isize && tc >= 0 && tc < map[tr as usize].len() as isize
}

fn can_go(map: &[Vec<char>], location: (usize, usize)) -> bool {
    let (r, c) = location;

    map[r][c] == '.' || map[r][c] == 'X'
}

fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|row| { println!("{}", row.iter().collect::<String>()); });
}

fn main() {
    let f = File::open("input.txt").expect("Could not open file.");
    let reader = BufReader::new(f);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut c = -1;
    let mut r = -1;
    for line in reader.lines() {
        let map_line = line.expect("Could not read input line.");

        if map_line.contains(['^', '>', 'v', '<']) {
            r = map.len() as isize;
            c = map_line.find(['^', '>', 'v', '<']).unwrap() as isize;
        }

        map.push(map_line.chars().collect());
    }

    println!(
        "Guard is starting position ({}, {}), direction is {}",
        r, c, map[r as usize][c as usize]
    );

    let mut positions_count = 1;
    loop {
        let (tr, tc) = next_step(&map, (r as usize, c as usize));

        if !within_lab(&map, (tr, tc)) {
            break;
        }

        if !can_go(&map, (tr as usize, tc as usize)) {
            turn_guard_right(&mut map, (r as usize, c as usize));
            continue;
        }

        if step_guard(&mut map, (r as usize, c as usize), (tr as usize, tc as usize)) {
            positions_count += 1;
        }
        (r, c) = (tr, tc);
        
        // print_map(&map);
    }

    println!("Positions count: {}", positions_count);
}
