use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_larger(a: [&i32; 3], b: [&i32; 3]) -> bool {
    let a: i32 = a.into_iter().sum();
    let b: i32 = b.into_iter().sum();
    return a < b;
}

fn main() -> io::Result<()> {
    let file = File::open("./src/day1.input")?;
    let reader = BufReader::new(file);
    let depths: Vec<i32> = reader.lines().
        filter(|item| item.is_ok()).
        map(|item| item.unwrap()).
        map(|item| item.parse::<i32>()).
        filter(|item| item.is_ok()).
        map(|item| item.unwrap()).
        collect();

    let mut count = 0;
    let mut prev: [&i32; 3] = [&depths[0], &depths[1], &depths[2]];
    let mut curr: [&i32; 3] = [&depths[1], &depths[2], &depths[3]];
    let mut idx = 3;

    for c in depths.iter().skip(4) {

        if is_larger(prev, curr) {
            count += 1;
        }

        prev[idx % 3] = curr[(idx - 1) % 3];
        curr[idx % 3] = c;

        idx += 1;
    }

    if is_larger(prev, curr) {
        count += 1;
    }

    println!("The answer is {}.  This was built by Rust:tm:", count);

    Ok(())
}



