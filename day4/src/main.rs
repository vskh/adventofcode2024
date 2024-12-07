use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_if_word_in_direction(
    word: &str,
    map: &[Vec<char>],
    r: usize,
    c: usize,
    direction: &(isize, isize),
) -> bool {
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

        if buf.len() == word.len() {
            break;
        }

        cr += direction.0;
        cc += direction.1;
    }

    buf == word
}

fn count_x_max_at_position(map: &[Vec<char>], r: usize, c: usize) -> usize {
    let rr = r as isize;
    let cc = c as isize;
    let combos = &[
        [
            ((rr - 1, cc - 1), (1, 1)),
            ((rr + 1, cc + 1), (-1, -1)),
            ((rr - 1, cc + 1), (1, -1)),
            ((rr + 1, cc - 1), (-1, 1)),
        ],
        // [
        //     ((rr, cc - 1), (0, 1)),
        //     ((rr, cc + 1), (0, -1)),
        //     ((rr - 1, cc), (1, 0)),
        //     ((rr + 1, cc), (-1, 0)),
        // ],
    ];

    combos
        .iter()
        .map(|x| {
            (x.map(|((sr, sc), direction)| {
                check_if_word_in_direction("MAS", map, sr as usize, sc as usize, &direction)
                    as usize
            })
            .iter()
            .sum::<usize>()
                > 1) as usize
        })
        .sum()
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
        sum += check_if_word_in_direction("XMAS", map, r, c, direction) as usize;
    }

    sum
}

fn calc_xmas_puzzle(map: &[Vec<char>]) -> (usize, usize) {
    let mut total_xmases = 0;
    let mut total_x_mases = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'X' {
                total_xmases += xmases_at_point(map, r, c);
            }
            if map[r][c] == 'A' {
                total_x_mases += count_x_max_at_position(map, r, c);
            }
        }
    }

    (total_xmases, total_x_mases)
}

fn main() {
    let f = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(f);

    let input: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.expect("Failed to read the input line.").chars().collect())
        .collect();

    let (total_xmases, total_x_mases) = calc_xmas_puzzle(&input);

    println!("Total XMAS encounters found: {}", total_xmases);
    println!("Total X-MAS encounters found: {}", total_x_mases);
}

#[cfg(test)]
mod test {
    use crate::calc_xmas_puzzle;

    #[test]
    fn calc_xmas_puzzle_works1() {
        let input = vec![
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 9));
    }

    #[test]
    fn calc_xmas_puzzle_works2() {
        let input = vec![
            vec!['.', '.', '.', '.', 'X', 'X', 'M', 'A', 'S', '.'],
            vec!['.', 'S', 'A', 'M', 'X', 'M', 'S', '.', '.', '.'],
            vec!['.', '.', '.', 'S', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', '.', 'M', 'S', '.', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', '.', 'M', 'M'],
            vec!['X', '.', '.', '.', '.', '.', 'X', 'A', '.', 'A'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', 'S'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', 'A'],
            vec!['.', '.', 'M', '.', 'M', '.', 'M', '.', 'M', 'M'],
            vec!['.', 'X', '.', 'X', '.', 'X', 'M', 'A', 'S', 'X'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (18, 3));
    }

    #[test]
    fn calc_xmas_puzzle_works3() {
        let input = vec![
            vec!['.', 'M', '.'],
            vec!['S', 'A', 'M'],
            vec!['.', 'S', '.'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 0));
    }

    #[test]
    fn calc_xmas_puzzle_works4() {
        let input = vec![
            vec!['M', 'M', 'S'],
            vec!['S', 'A', 'M'],
            vec!['M', 'S', 'S'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 1));
    }

    #[test]
    fn calc_xmas_puzzle_works5() {
        let input = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 1));
    }

    #[test]
    fn calc_xmas_puzzle_works6() {
        let input = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 0));
    }

    #[test]
    fn calc_xmas_puzzle_works7() {
        let input = vec![
            vec!['M', 'M', '.'],
            vec!['.', 'A', '.'],
            vec!['.', 'S', 'S'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 0));
    }

    #[test]
    fn calc_xmas_puzzle_works8() {
        let input = vec![
            vec!['S', 'M', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', 'S', 'M'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 1));
    }

    #[test]
    fn calc_xmas_puzzle_works9() {
        let input = vec![
            vec!['S', 'M', 'S'],
            vec!['A', 'A', 'A'],
            vec!['M', 'S', 'M'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 1));
    }

    #[test]
    fn calc_xmas_puzzle_works10() {
        let input = vec![
            vec!['M', 'S', 'M', 'S'],
            vec!['M', 'A', 'A', '.'],
            vec!['S', 'M', 'S', 'M'],
        ];

        assert_eq!(calc_xmas_puzzle(&input), (0, 2));
    }
}
