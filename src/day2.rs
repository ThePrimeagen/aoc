use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct InputLine {
    direction: String,
    count: i64,
}

struct Output {
    depth: i64,
    aim: i64,
    x: i64,
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./src/day2.input")?;
    let output = BufReader::new(file).lines().
        filter_map(|line| {
            let line = line.unwrap();
            println!("Parsing line: {}", line);
            match line.split_once(" ") {
                Some((dir, count)) => {
                    println!("Got Some {} {}", dir, count);
                    return Some(
                        InputLine {
                            direction: dir.to_string(),
                            count: count.parse::<i64>().unwrap()
                        }
                    );
                },
                None => {
                    println!("Got nonya");
                    return None;
                }
            }
        }).
        fold(Output{aim: 0, depth: 0, x: 0}, |mut result, line| {
            println!("{:?}", line);
            match line.direction.as_str() {
                "forward" => {
                    result.x += line.count;
                    result.depth += result.aim * line.count;
                }
                "up" => result.aim -= line.count,
                "down" => result.aim += line.count,
                // I want to get rid of this.  But that's later
                _ => {}
            }

            return result;
        });

    println!("Output {}. Brought to you by Rustlang:tm:", output.depth * output.x);

    return Ok(());
}
