use std::cmp::{max, min};
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct Fissure {
    x: Range<usize>,
    y: Range<usize>,
}

fn to_point(s: &str) -> Result<(usize, usize), std::io::Error> {
    let (x, y) = s.split_once(",").unwrap();
    let x = x.parse::<usize>().unwrap();
    let y = y.parse::<usize>().unwrap();

    return Ok((x, y));
}

impl FromStr for Fissure {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        let (x, y) = to_point(start)?;
        let (x2, y2) = to_point(end)?;

        let fissure = Fissure {
            x: min(x, x2)..(max(x, x2) + 1),
            y: min(y, y2)..(max(y, y2) + 1),
        };

        return Ok(fissure);
    }
}

impl Fissure {
    fn is_straight(&self) -> bool {
        return self.x.start + 1 == self.x.end || self.y.start + 1 == self.y.end;
    }
}

fn main() {
    let fissures: Vec<Fissure> = include_str!("day5.input")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    for f in &fissures {
        println!("{:?}", f);
    }

    let (max_x, max_y) = fissures.iter().fold((0, 0), |mut point, f| {
        if point.0 < f.x.end {
            point.0 = f.x.end;
        }
        if point.1 < f.y.end {
            point.1 = f.y.end;
        }
        return point;
    });

    let (_, count) = fissures.iter().filter(|f| f.is_straight()).fold(
        (vec![vec![0; max_x]; max_y], 0),
        |bnc, f| {
            let (mut board, mut count) = bnc;
            println!("about to {:?}", f);
            for x in f.x.start..f.x.end {
                for y in f.y.start..f.y.end {
                    println!("Point ({}, {})", x, y);
                    board[y][x] += 1;
                    if board[y][x] == 2 {
                        count += 1;
                    }
                }
            }
            return (board, count);
        },
    );

    println!("Count {}.  Powered by Rustlang:tm:", count);
}


