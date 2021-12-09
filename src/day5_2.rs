use std::cmp::max;
use std::str::FromStr;

#[derive(Debug)]
struct FakeRange {
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Fissure {
    x: FakeRange,
    y: FakeRange,
}

fn to_point(s: &str) -> Result<(i32, i32), std::io::Error> {
    let (x, y) = s.split_once(",").unwrap();
    let x = x.parse::<i32>().unwrap();
    let y = y.parse::<i32>().unwrap();

    return Ok((x, y));
}

fn to_range(range: &FakeRange) -> Box<dyn Iterator<Item = i32>> {
    let a1 = range.start;
    let a2 = range.end;

    if a1 > a2 {
        return Box::new((a2..=a1).rev());
    }

    return Box::new(a1..=a2);
}

impl FromStr for Fissure {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        let (x, y) = to_point(start)?;
        let (x2, y2) = to_point(end)?;

        let fissure = Fissure {
            x: FakeRange {start: x, end: x2},
            y: FakeRange {start: y, end: y2},
        };

        return Ok(fissure);
    }
}

impl Fissure {
    fn is_straight(&self) -> bool {
        return self.x.start == self.x.end || self.y.start == self.y.end;
    }

    fn is_45(&self) -> bool {
        return ((self.x.end - self.x.start) as i32).abs()
            == ((self.y.end - self.y.start) as i32).abs();
    }
}

fn print_board(board: &Vec<Vec<i32>>) {
    /*
    for line in board {
        println!("{:?}", line)
    }
    */
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
        let max_x = max(f.x.end, f.x.start);
        let max_y = max(f.y.end, f.y.start);

        if point.0 < max_x {
            point.0 = max_x;
        }
        if point.1 < max_y {
            point.1 = max_y;
        }
        return point;
    });
    let max_x = max_x as usize + 1;
    let max_y = max_y as usize + 1;

    println!("Size {} {}", max_x, max_y);

    let (board, count) = fissures
        .iter()
        .filter(|f| f.is_straight())
        .fold((vec![vec![0; max_y]; max_x], 0), |bnc, f| {
            let (mut board, mut count) = bnc;
            println!("fiss {:?}", f);

            for x in to_range(&f.x) {
                for y in to_range(&f.y) {
                    println!("  p({}, {})", x, y);
                    board[y as usize][x as usize] += 1;
                    if board[y as usize][x as usize] == 2 {
                        count += 1;
                    }
                }
            }

            print_board(&board);

            return (board, count);
        });

    let (_, count) = fissures
        .iter()
        .filter(|f| f.is_45())
        .flat_map(|f| {
            println!("Creating Diag {:?} - {:?}", f.x, f.y);
            to_range(&f.x).zip(to_range(&f.y))
        })
        .fold((board, count), |bnc, it| {
            let (x, y) = it;
            let (mut board, mut count) = bnc;

            println!("diag ({}, {})", x, y);
            print_board(&board);

            board[y as usize][x as usize] += 1;
            if board[y as usize][x as usize] == 2 {
                count += 1;
            }

            return (board, count);
        });

    println!("Count {}.  Powered by Rustlang:tm:", count);
}

