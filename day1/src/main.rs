use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let list_items = line?
            .split_ascii_whitespace()
            .map(|el| el.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        list1.push(list_items[0]);
        list2.push(list_items[1]);
    }

    list1.sort();
    list2.sort();

    let mut total_distance = 0;
    for (&l1n, &l2n) in list1.iter().zip(list2.iter()) {
        // println!("{}:{}", l1n, l2n);
        let diff = u32::abs_diff(l1n, l2n);
        total_distance += diff;
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}
