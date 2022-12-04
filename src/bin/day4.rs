use std::str::FromStr;

use anyhow::Result;

struct Task {
    start: usize,
    end: usize,
}

impl Task {
    pub fn contains(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    pub fn contains_some(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.start ||
            self.end >= other.end && self.start <= other.end;
    }
}

struct Tasks {
    left: Task,
    right: Task,
}

impl Tasks {
    pub fn one_task_contains_the_other(&self) -> bool {
        return self.left.contains(&self.right) ||
            self.right.contains(&self.left);
    }

    pub fn one_task_contains_some_of_the_other(&self) -> bool {
        return self.left.contains_some(&self.right) ||
            self.right.contains_some(&self.left);
    }
}

impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").expect("aoc is not a liar");
        return Ok(Tasks {
            left: left.parse()?,
            right: right.parse()?,
        });
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").expect("this has to exist or aoc is screwing me");
        return Ok(Task {
            start: a.parse()?,
            end: b.parse()?,
        });
    }
}

fn main() -> Result<()> {
    println!("help me daddy {}",  std::fs::read_to_string("./src/bin/day4.prod")?
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.one_task_contains_some_of_the_other())
        .count());

    return Ok(());
}

