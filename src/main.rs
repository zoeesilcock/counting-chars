use std::collections::HashMap;
use std::process;

fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {err}");
        process::exit(1);
    });

    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        let entry = char_counts.entry(c).or_insert(0);
        *entry += 1;
    }

    let mut result: Vec<(&char, &u32)> = char_counts.iter().collect();
    result.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", result);
}
