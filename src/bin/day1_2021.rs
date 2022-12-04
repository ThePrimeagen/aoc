use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/bin/day1.input")?;
    let reader = BufReader::new(file);

    let depths: Vec<i32> = reader
        .lines()
        .flat_map(|x| x)
        .flat_map(|item| item.parse::<i32>())
        .collect();

    let mut count = 0;
    let mut prev: i32 = depths[0];

    for c in depths.iter().skip(1) {
        if prev < *c {
            count += 1;
        }

        prev = *c;
    }

    println!("The answer is {}.  This was built by Rust:tm:", count);

    Ok(())
}
