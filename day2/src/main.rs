use std::{
    cmp::{max, Ordering},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn determine_ordering(report: &[u32]) -> Ordering {
    let counts = report.windows(2).fold(
        HashMap::from([
            (Ordering::Less, 0),
            (Ordering::Equal, 0),
            (Ordering::Greater, 0),
        ]),
        |mut acc, el| {
            acc.entry(match (el[0], el[1]) {
                (f, s) if f < s => Ordering::Less,
                (f, s) if f > s => Ordering::Greater,
                _ => Ordering::Equal,
            })
            .and_modify(|v| *v += 1);

            acc
        },
    );

    counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(&order, _)| order)
        .unwrap()
}

fn is_safe_report(report: &[u32]) -> bool {
    let ordering = determine_ordering(report);
    let mut last_level = report[0];

    for &curr_level in report.iter().skip(1) {
        if (ordering == Ordering::Less && curr_level <= last_level)
            || (ordering == Ordering::Greater && curr_level >= last_level)
            || ordering == Ordering::Equal
            || !(1..=3).contains(&last_level.abs_diff(curr_level))
        {
            return false;
        }

        last_level = curr_level;
    }

    true
}

fn is_safe_report_dampened(report: &[u32]) -> bool {
    let ordering = determine_ordering(report);
    let mut last_level = report[0];

    for (idx, &curr_level) in report.iter().enumerate().skip(1) {
        let diff = last_level.abs_diff(curr_level);

        if (ordering == Ordering::Less && curr_level <= last_level)
            || (ordering == Ordering::Greater && curr_level >= last_level)
            || ordering == Ordering::Equal
            || !(1..=3).contains(&diff)
        {
            // println!("Unsafe without dampening: {:?}", report);

            let try_splits = [max(0, idx - 1), idx];

            for split in try_splits {
                let try_fixed_report = [&report[0..split], &report[split + 1..]].concat();
                let is_safe_now = is_safe_report(&try_fixed_report);

                if is_safe_now {
                    // println!("     But dampening helps: {:?}", try_fixed_report);
                    return is_safe_now;
                }
            }

            return false;
        }

        last_level = curr_level;
    }

    true
}

fn main() {
    let input_file = File::open("input.txt").expect("Failed to open file.");
    let reader = BufReader::new(input_file);

    let mut total_reports_count = 0;
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

        safe_reports_count += is_safe as u32;
        safe_reports_count_dampened += is_safe_dampened as u32;
        total_reports_count += 1;
    }

    println!("Total reports count: {}", total_reports_count);
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
    use std::cmp::Ordering;

    use crate::{determine_ordering, is_safe_report, is_safe_report_dampened};

    #[test]
    fn is_safe_report_works() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1]));
        assert!(!is_safe_report(&[1, 2, 7, 8, 9]));
        assert!(!is_safe_report(&[9, 7, 6, 2, 1]));
        assert!(!is_safe_report(&[1, 3, 2, 4, 5]));
        assert!(!is_safe_report(&[8, 6, 4, 4, 1]));
        assert!(is_safe_report(&[1, 3, 6, 7, 9]));

        assert!(!is_safe_report(&[1, 1, 2, 3, 4]));
        assert!(!is_safe_report(&[71, 69, 70, 71, 72, 75]));
        assert!(!is_safe_report(&[2, 5, 4, 3, 2]));
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
        assert!(is_safe_report_dampened(&[1, 1, 2, 3, 4]));
        assert!(is_safe_report_dampened(&[2, 5, 4, 3, 2]));
        assert!(is_safe_report_dampened(&[71, 69, 70, 71, 72, 75]));
        assert!(is_safe_report_dampened(&[5, 8, 4, 3, 2, 1]));
    }

    #[test]
    fn determine_ordering_works() {
        assert_eq!(determine_ordering(&[1, 2, 3, 4, 5]), Ordering::Less);
        assert_eq!(determine_ordering(&[5, 4, 3, 2, 1]), Ordering::Greater);
        assert_eq!(determine_ordering(&[1, 1, 1, 1, 1]), Ordering::Equal);
        assert_eq!(determine_ordering(&[1, 2, 3, 2, 1, 0]), Ordering::Greater);
    }
}
