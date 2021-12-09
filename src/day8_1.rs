#![feature(int_abs_diff)]

fn project(s: &str) -> u64 {
    println!("'{}'", s);
    return s.chars().map(|x| {
        // step 1 convert char => byte
        // x - 'a'
        // return 10pow(FOO[x - 'a'])

        0x1 << (x as u8 - 'a' as u8)
    }).fold(0, |acc, x| acc + x);
}

fn main() {

    let count = include_str!("day8.input").trim().lines().fold(0, |acc, line| {
        let (input, output) = line.split_once(" | ").unwrap();
        let found: Vec<u64> = input
            .split(" ")
            .filter(|seg| [4, 3, 2, 7].contains(&seg.len()))
            .map(project)
            .collect();

        println!("{:?}", found);
        output
            .split(" ")
            .map(str::trim)
            .map(project)
            .filter(|o| {
                println!("Testing {} against {:?} = {}", o, found, found.contains(o));
                found.contains(o)
            })
        .count() + acc
    });

    println!("{} Powered by RustLang:tm:", count);
}

