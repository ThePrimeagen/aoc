use std::{fs, str::FromStr};

use anyhow::Result;

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<Option<char>>>,
}

impl Crane {
    fn move_me_daddy_crane(&mut self, m: &Move) {
        for _ in 0..m.count {
            let item = self.stack[m.from].pop().expect("should exist");
            self.stack[m.to].push(item);

        }
    }

    fn move_me_daddy_9001(&mut self, m: &Move) {
        let out_items = (0..m.count)
            .filter_map(|_| self.stack[m.from].pop())
            .collect::<Vec<_>>();
        for item in out_items.into_iter().rev() {
            self.stack[m.to].push(item);

        }
    }

    fn print_bottom(&self) {
        print!("Stack: ");
        for stack in &self.stack {
            print!("{}", stack.last().expect("i like it dangerous").expect("very dangerous"));
        }

        println!();
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut idx = 0;
            let mut crane_idx = 0;

            while idx < line.len() {
                if stack.len() <= crane_idx {
                    stack.push(Vec::new());
                }

                if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx + 1).expect("must exist");
                    stack[crane_idx].push(Some(c));
                }

                idx += 4;
                crane_idx += 1;
            }
        }

        return Ok(Crane { stack });
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Move
{
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let count = iter.next().expect("must exist");
        let from = iter.next().expect("must exist") - 1;
        let to = iter.next().expect("must exist") - 1;
        return Move {
            count, from, to
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //"move 1 from 2 to 1";
        return Ok(s
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Move>());
    }
}

fn crane_time_me_daddy(file: String) -> Result<()> {

    let (crane, moves) = file.split_once("\n\n").expect("aoc input to not break");

    let mut crane = crane.parse::<Crane>()?;
    let moves = moves
        .lines()
        .filter(|x| !x.is_empty())
        .filter_map(|x| x.parse::<Move>().ok())
        .collect::<Vec<Move>>();

    for m in moves {
        crane.move_me_daddy_9001(&m);
    }

    crane.print_bottom();

    return Ok(());
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./src/bin/day5.test")?;
    crane_time_me_daddy(file)?;

    return Ok(());
}

#[cfg(test)]
mod test {
    use std::fs;

    use anyhow::Result;

    use crate::crane_time_me_daddy;

    #[test]
    fn test_crane_parse() -> Result<()> {
        let file = fs::read_to_string("./src/bin/day5.prod")?;
        crane_time_me_daddy(file)?;

        return Ok(());
    }
}
