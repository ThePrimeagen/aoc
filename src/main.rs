#![feature(int_abs_diff)]

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
// consht mut data: Vec<Vec<u32>> = include_str!("day10.test");

lazy_static! {
    static ref SCORES: HashMap<char, u64> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    static ref OPENINGS: HashSet<char> = HashSet::from([
        '(',
        '[',
        '{',
        '<',
    ]);

    // Going to the hacking session
    static ref OPPOSING: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
}

fn get_stack(line: &str) -> Option<Vec<char>> {
    let mut stack: Vec<char> = vec![];
    let mut is_corrupted = false;

    for c in line.chars() {
        if OPENINGS.contains(&c) {
            stack.push(c);
        } else {
            if let Some(last) = stack.pop() {
                if OPPOSING[&last] != c {
                    is_corrupted = true;
                    break;
                }
            }
        }
    }

    if is_corrupted {
        return None;
    }
    return Some(stack)
}

fn main() {
    let mut count: Vec<u64> = include_str!("day10.input")
        .trim()
        .lines()
        .filter_map(get_stack)
        .map(|missing|
            missing.iter().rev().fold(0, |acc, x| {
                (acc * 5) + SCORES[x]
            })
        )
        .collect();

    count.sort();

    println!("{:?} Powered by RustLang:tm:", count[count.len() / 2]);
}
