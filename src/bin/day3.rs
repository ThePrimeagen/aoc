#![feature(iter_array_chunks)]

use std::{str::FromStr, collections::{HashSet, HashMap}, rc::Rc};

use anyhow::Result;

const START_LOWER: u8 = b'a' - 1;
const START_UPPER: u8 = b'A' - 1;

struct Item {
    value: usize,
}

impl TryFrom<&u8> for Item {
    type Error = ();

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        let value = if *value > b'a' {
            *value as u8 - b'a' + 1
        } else {
            *value as u8 - b'A' + 27
        };

        Ok(Self { value: value as usize })
    }
}

fn check_group(chunk: &[&str]) -> usize {
    let mut occurrences: [u8; 53] = [0; 53];
    let mut sum = 0;

    for (row_index, sack) in chunk.iter().enumerate() {
        for element in sack.as_bytes() {
            let item = Item::try_from(element).unwrap();

            occurrences[item.value] |= 1 << row_index;
            if occurrences[item.value] == 0b111 {
                sum += item.value;
                break;
            }
        }
    }
    return sum;
}

fn main() -> Result<()> {
    let foo = std::fs::read_to_string("src/bin/day3.prod")?
        .lines()
        .array_chunks::<3>()
        .flat_map(|line| {
            return line
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3)
        })
        .map(|c| c.0)
        .map(|c| {
            let value = if c.is_ascii_lowercase() {
                c as u8 - START_LOWER
            } else {
                c as u8 - START_UPPER + 26
            } as u32;

            return value;
         })
        .sum::<u32>();


    println!("{}", foo);

    return Ok(())
}

