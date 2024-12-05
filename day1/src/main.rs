use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut list1 = BinaryHeap::new();
    let mut list2 = BinaryHeap::new();

    for line in reader.lines() {
        let list_items = line?
            .split_ascii_whitespace()
            .map(|el| el.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        list1.push(list_items[0]);
        list2.push(list_items[1]);
    }

    let mut total_distance = 0;
    while let (Some(l1n), Some(l2n)) = (list1.pop(), list2.pop()) {
        println!("{}:{}", l1n, l2n);
        let diff = u32::abs_diff(l1n, l2n);
        total_distance += diff;
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}
