use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Rule(u32, u32);

impl Rule {
    fn new(first: u32, second: u32) -> Self {
        Self(first, second)
    }

    fn parse(input: &str) -> Result<Self, String> {
        let parsed: Result<Vec<_>, _> = input
            .split('|')
            .map(|el| el.parse::<u32>().map_err(|e| format!("Error: {}", e)))
            .collect();

        let parts = parsed?;
        Ok(Self::new(*parts.first().unwrap(), *parts.last().unwrap()))
    }

    fn is_satisfied_for_update(&self, update: &[u32]) -> bool {
        let first_pos = update.iter().position(|el| *el == self.0);
        let second_pos = update.iter().position(|el| *el == self.1);

        !matches!((first_pos, second_pos), (Some(f), Some(s)) if f > s)
    }
}

fn main() {
    let f = File::open("input.txt").expect("Failed to open the input file.");
    let reader = BufReader::new(f);

    let mut rules = Vec::new();
    let mut header_read = true;
    let mut middle_pages_sum = 0;
    for line_result in reader.lines() {
        if header_read {
            let rule_line = line_result.expect("Failed to read from the input file.");

            if rule_line.is_empty() {
                header_read = false;
                continue;
            }

            rules.push(
                Rule::parse(rule_line.as_str())
                    .unwrap_or_else(|_| panic!("Failed to parse rule: {}", rule_line)),
            );
        } else {
            let update_line = line_result.expect("Failed to read from the input file.");

            let update_result: Result<Vec<u32>, String> = update_line
                .split(',')
                .map(|el| {
                    el.parse::<u32>()
                        .map_err(|e| format!("Failed to parse update: {}", e))
                })
                .collect();

            let update = update_result.unwrap_or_else(|e| panic!("Failed to parse update line '{}': {}", update_line, e));

            if rules.iter().all(|r| r.is_satisfied_for_update(update.as_slice())) {
                assert!(update.len() % 2 == 1, "Update {:?} has even number of pages, can't determine the middle page number.", update);

                middle_pages_sum += update[update.len() / 2];
            }
        }
    }
    println!("Sum of middle page numbers of right ordered updates: {}", middle_pages_sum);
}
