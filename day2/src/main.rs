use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_safe_report(report: &[u32]) -> bool {
    let mut last_level = report[0];
    let ascending = report[0] < report[1];
    for &curr_level in report.iter().skip(1) {
        if (ascending && curr_level < last_level) || (!ascending && curr_level > last_level) {
            return false;
        }

        if !(1..=3).contains(&last_level.abs_diff(curr_level)) {
            return false;
        }

        last_level = curr_level;
    }

    true
}

fn is_safe_report_dampened(report: &[u32]) -> bool {
    let mut last_level = report[0];
    let ascending = report[0] < report[1];
    for (idx, &curr_level) in report.iter().enumerate().skip(1) {
        if (ascending && curr_level < last_level) || (!ascending && curr_level > last_level) {
            return is_safe_report(&[&report[0..idx], &report[idx + 1..]].concat());
        }

        let diff = last_level.abs_diff(curr_level);
        if !(1..=3).contains(&diff) {
            return is_safe_report(&[&report[0..idx], &report[idx + 1..]].concat());
        }

        last_level = curr_level;
    }

    true
}

fn main() {
    let input_file = File::open("input.txt").expect("Failed to open file.");
    let reader = BufReader::new(input_file);

    let mut safe_reports_count = 0;
    let mut safe_reports_count_dampened = 0;
    for line in reader.lines() {
        let report: Vec<u32> = line
            .expect("Failed to read report line.")
            .split_whitespace()
            .map(|el| el.parse::<u32>().unwrap())
            .collect();

        let is_safe = is_safe_report(&report);
        let is_safe_dampened = is_safe_report_dampened(&report);

        if is_safe != is_safe_dampened {
            println!("Dampening helps: {:?}", report);
        }

        safe_reports_count += is_safe as u32;
        safe_reports_count_dampened += is_safe_dampened as u32;
    }

    println!(
        "Number of safe reports without dampening: {}",
        safe_reports_count
    );
    println!(
        "Number of safe reports with dampening: {}",
        safe_reports_count_dampened
    );
}

#[cfg(test)]
mod test {
    use crate::{is_safe_report, is_safe_report_dampened};

    #[test]
    fn is_safe_report_works() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1]));
        assert!(!is_safe_report(&[1, 2, 7, 8, 9]));
        assert!(!is_safe_report(&[9, 7, 6, 2, 1]));
        assert!(!is_safe_report(&[1, 3, 2, 4, 5]));
        assert!(!is_safe_report(&[8, 6, 4, 4, 1]));
        assert!(is_safe_report(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn is_safe_report_dampened_works() {
        assert!(is_safe_report_dampened(&[7, 6, 4, 2, 1]));
        assert!(!is_safe_report_dampened(&[1, 2, 7, 8, 9]));
        assert!(!is_safe_report_dampened(&[9, 7, 6, 2, 1]));
        assert!(is_safe_report_dampened(&[1, 3, 2, 4, 5]));
        assert!(is_safe_report_dampened(&[8, 6, 4, 4, 1]));
        assert!(is_safe_report_dampened(&[1, 3, 6, 7, 9]));

        assert!(!is_safe_report_dampened(&[61, 64, 67, 66, 68, 67]));
        assert!(is_safe_report_dampened(&[75, 72, 71, 68, 66, 65, 62, 56]));
    }
}
