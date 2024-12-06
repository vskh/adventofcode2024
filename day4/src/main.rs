use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_if_xmas_in_direction(map: &[Vec<char>], r: usize, c: usize, direction: &(isize, isize)) -> bool {
    let mut cr = r as isize;
    let mut cc = c as isize;
    let mut buf = String::new();

    loop {
        if cr < 0 || cr as usize >= map.len() {
            break;
        }

        if cc < 0 || cc as usize >= map[cr as usize].len() {
            break;
        }

        buf.push(map[cr as usize][cc as usize]);

        if buf.len() == 4 {
            break;
        }

        cr += direction.0;
        cc += direction.1;
    }

    buf == "XMAS"
}

fn xmases_at_point(map: &[Vec<char>], r: usize, c: usize) -> usize {
    let directions: &[(isize, isize)] = &[
        /* right */ (1, 0),
        /* left */ (-1, 0),
        /* up */ (0, -1),
        /* down */ (0, 1),
        /* diag/up-right */ (1, 1),
        /* diag/up-left */ (-1, 1),
        /* diag/down-left */ (-1, -1),
        /* diag/down-right */ (1, -1),
    ];

    let mut sum = 0;

    for direction in directions {
        sum += check_if_xmas_in_direction(map, r, c, direction) as usize;
    }

    sum
}

fn main() {
    let f = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(f);

    let input: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.expect("Failed to read the input line.").chars().collect())
        .collect();

    let mut total_xmases = 0;
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] == 'X' {
                total_xmases += xmases_at_point(&input, r, c);
            }
        }
    }

    println!("Total XMAS encounters found: {}", total_xmases);
}
