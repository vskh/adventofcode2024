use std::{fs::File, io::{BufReader, Read}};

fn try_parse_and_mul(mem: &str) -> Option<u32> {
    let mut p = mem;
    if !p.starts_with("mul(") {
        // println!("!mul(");
        return None;
    }

    p = &p[4..]; // skip prefix

    let mut len = 0;
    for ch in p.chars() {
        if !ch.is_ascii_digit() {
            break;
        }

        len += 1;
    }

    if len == 0 {
        // println!("!arg1");
        return None; // could not parse the first argument
    }

    let arg1: u32 = p[0..len].parse().ok()?;
    p = &p[len..];

    if !p.starts_with(",") {
        // println!("!,");
        return None; // expect comma between arguments
    }

    p = &p[1..];

    len = 0;
    for ch in p.chars() {
        if !ch.is_ascii_digit() {
            break;
        }

        len += 1;
    }

    if len == 0 {
        // println!("!arg2");
        return None; // could not parse the first argument
    }

    let arg2: u32 = p[0..len].parse().ok()?;
    p = &p[len..];

    if !p.starts_with(")") {
        // println!("!)");
        return None
    }

    Some(arg1 * arg2)
}

fn main() {
    let f = File::open("input.txt").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);

    let mut mem = String::new();
    reader.read_to_string(&mut mem).expect("Failed to read the input file.");

    let mut sum_products = 0;
    for (i, ch) in mem.char_indices() {
        if ch == 'm' {
            sum_products += try_parse_and_mul(&mem[i..]).unwrap_or(0);
        }
    }

    println!("Sum of products: {}", sum_products);
}

#[cfg(test)]
mod test {
    use crate::try_parse_and_mul;

    #[test]
    fn try_parse_mul_works() {
        assert!(try_parse_and_mul("mul(771,307(").is_none());
        assert!(matches!(try_parse_and_mul("mul(301,529)who(86,180)"), Some(p) if p == 301 * 529));
    }
}