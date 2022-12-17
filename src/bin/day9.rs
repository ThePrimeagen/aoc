use std::{collections::HashSet, f64::consts::SQRT_2};

use anyhow::Result;
use itertools::Itertools;

const DIRECTIONS: [Dir; 8] = [
    Dir(0, 1),
    Dir(1, 0),
    Dir(-1, 0),
    Dir(0, -1),

    Dir(1, 1),
    Dir(1, -1),
    Dir(-1, 1),
    Dir(-1, -1),
];

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Dir(isize, isize);

impl Dir {
    pub fn dist(&self, other: &Dir) -> f64 {
        let x_diff = self.0.abs_diff(other.0);
        let y_diff = self.1.abs_diff(other.1);

        return f64::sqrt((x_diff * x_diff) as f64 + (y_diff * y_diff) as f64);
    }
}


#[derive(Debug)]
struct Info {
    nodes: Vec<Dir>,
    seen: HashSet<Dir>,
}

impl Info {
    pub fn new(size: usize) -> Self {
        let mut info = Info {
            nodes: vec![Dir(0, 0); size],
            seen: HashSet::new()
        };

        info.seen.insert(Dir(0, 0));

        return info;
    }
}

impl Info {
    pub fn add(&mut self, direction: &Dir) {
        self.nodes[0].0 += direction.0;
        self.nodes[0].1 += direction.1;

        (1..self.nodes.len())
            .for_each(|x| {
                let (left, right) = self.nodes.split_at_mut(x);
                let head = left.last_mut().expect("at least one");
                let follower = right.first_mut().expect("at least one");

                let dist = head.dist(&follower);
                if dist > SQRT_2 {
                    let smallest: (Dir, _) = DIRECTIONS
                        .iter()
                        .map(|Dir(x, y)| {
                            let next_pos = Dir(follower.0 + x, follower.1 + y);
                            let dist = next_pos.dist(&head);
                            return (next_pos, dist);
                        })
                    .min_by(|x, y| {
                        return x.1.partial_cmp(&y.1).expect("NaN me daddy");
                    }).expect("there to always be a solution");

                    follower.0 = smallest.0.0;
                    follower.1 = smallest.0.1;
                }
            });

        self.seen.insert(self.nodes.last().expect("its a trap").clone());
    }
}

fn to_direction(dir: &str) -> Dir {
    return match dir {
        "R" => Dir(1, 0),
        "L" => Dir(-1, 0),
        "U" => Dir(0, 1),
        "D" => Dir(0, -1),
        _ => unreachable!("What the hell is wrong with you")
    };
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./src/bin/day9.prod")?;

    let info = file
        .lines()
        .fold(Info::new(10), |mut info, line| {
            let (dir, amount) = line.split_once(" ").unwrap();
            let amount = amount.parse::<isize>().unwrap();
            let dir = to_direction(dir);

            (0..amount).for_each(|_| {
                info.add(&dir);
            });

            return info
        });

    println!("info: {:?} {}", info, info.seen.len());

    return Ok(());
}

