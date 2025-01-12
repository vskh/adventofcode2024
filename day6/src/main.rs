use std::fs::File;
use std::io::{BufReader, Read};

fn read_map_from_str(s: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut c = -1;
    let mut r = -1;
    for map_line in s.split_terminator('\n') {
        if map_line.contains(['^', '>', 'v', '<']) {
            r = map.len() as isize;
            c = map_line.find(['^', '>', 'v', '<']).unwrap() as isize;
        }

        map.push(map_line.chars().collect());
    }

    (map, (r, c))
}

#[allow(unused)]
fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

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
        map[tr][tc],
        tr,
        tc,
        if unvisited { "new" } else { "has been before" }
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

    map[r][c] != '#'
}

fn run_guard_till_exit(map: &mut [Vec<char>], location: (usize, usize)) -> (usize, usize) {
    let mut r: isize = location.0 as isize;
    let mut c: isize = location.1 as isize;

    let mut positions_count = 1;
    let mut loops_count = 0;
    loop {
        let (nr, nc) = next_step(map, (r as usize, c as usize));

        if !within_lab(map, (nr, nc)) {
            break;
        }

        loops_count += is_loopable_perimeter(map, (r as usize, c as usize)) as usize;

        if !can_go(map, (nr as usize, nc as usize)) {
            turn_guard_right(map, (r as usize, c as usize));
            continue;
        }

        if step_guard(map, (r as usize, c as usize), (nr as usize, nc as usize)) {
            positions_count += 1;
        }
        (r, c) = (nr, nc);

        // print_map(map);
    }

    (positions_count, loops_count)
}

fn is_loopable_perimeter(map: &[Vec<char>], location: (usize, usize)) -> bool {
    let (sr, sc) = location;

    let mut turns = 0;

    // presume that current direction is caused by turn to avoid obstacle
    let mut direction = match map[sr][sc] {
        '^' => '<',
        '>' => '^',
        'v' => '>',
        '<' => 'v',
        _ => panic!("Wrong direction: {}", map[sr][sc]),
    };

    let mut r = sr;
    let mut c = sc;
    while turns <= 4 {
        direction = match direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Wrong direction: {}", direction),
        };
        turns += 1;

        let (dr, dc) = match direction {
            '^' => (-1_isize, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1_isize),
            _ => panic!("Wrong direction: {}", direction),
        };

        loop {
            let (nr, nc) = (r as isize + dr, c as isize + dc);

            if !within_lab(map, (nr, nc)) {
                return false;
            }

            if !can_go(map, (nr as usize, nc as usize)) {
                break;
            }

            r = nr as usize;
            c = nc as usize;

            if r == sr && c == sc {
                return true;
            }
        }

    }

    false
}

fn main() {
    let f = File::open("input.txt").expect("Could not open file.");
    let mut reader = BufReader::new(f);

    let mut map_string = String::new();
    reader
        .read_to_string(&mut map_string)
        .expect("Failed to read the input file.");
    let (mut map, (r, c)) = read_map_from_str(map_string.as_str());

    println!(
        "Guard is starting position ({}, {}), direction is {}",
        r, c, map[r as usize][c as usize]
    );

    assert!(r >= 0 && c >= 0, "Guard is expected to be within the lab!");

    let (positions_count, loops_count) = run_guard_till_exit(&mut map, (r as usize, c as usize));

    println!("Positions count: {}", positions_count);
    println!("Possible loops count: {}", loops_count);
}

#[cfg(test)]
mod test {
    use crate::{read_map_from_str, run_guard_till_exit};

    #[test]
    fn read_map_from_str_works() {
        let (map, (r, c)) = read_map_from_str(
            "....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...",
        );

        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
        assert_eq!((r, c), (6, 4));
    }

    #[test]
    fn run_guard_till_exit_works() {
        let (mut map, (r, c)) = read_map_from_str(
            "....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...",
        );

        let (positions_count, loops_count) = run_guard_till_exit(&mut map, (r as usize, c as usize));

        assert_eq!(positions_count, 41);
        assert_eq!(loops_count, 6);
    }

    mod is_loopable_perimeter {
        use crate::{is_loopable_perimeter, read_map_from_str};

        #[test]
        fn if_hit_lab_wall_then_false() {
            let (map, (r, c)) = read_map_from_str(
                ".....\n\
                #<....",
            );

            assert!(!is_loopable_perimeter(&map, (r as usize, c as usize)));
        }

        #[test]
        fn if_ran_out_of_turns_then_false() {
            let (map, (r, c)) = read_map_from_str(
                ".#...\n\
                 ...#.\n\
                 #<...\n\
                 #....\n\
                 ..#...",
            );

            assert!(!is_loopable_perimeter(&map, (r as usize, c as usize)));
        }

        #[test]
        fn if_loop_then_true() {
            let (map, (r, c)) = read_map_from_str(
                ".#...\n\
                 ...#.\n\
                 #<...\n\
                 ..#..",
            );

            assert!(is_loopable_perimeter(&map, (r as usize, c as usize)));
        }

        #[test]
        fn if_inplace_turns_then_true() {
            let (map, (r, c)) = read_map_from_str(
                ".#...\n\
                 #<#..\n\
                 .#...",
            );

            assert!(is_loopable_perimeter(&map, (r as usize, c as usize)));

        }
        #[test]
        fn if_back_and_force_then_true() {
            let (map, (r, c)) = read_map_from_str(
                ".#...\n\
                 #<..#\n\
                 ...#.",
            );

            assert!(is_loopable_perimeter(&map, (r as usize, c as usize)));
        }

        #[test]
        fn sample_map_true() {
            let (map, (r, c)) = read_map_from_str(
                "....#.....\n\
                .........#\n\
                ..........\n\
                ..#.......\n\
                .......#..\n\
                ..........\n\
                .#..^.....\n\
                ........#.\n\
                #.........\n\
                ......#...",
            );

            assert!(is_loopable_perimeter(&map, (r as usize, c as usize)));
        }
    }
}
