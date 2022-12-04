use std::{str::FromStr, rc::Rc};

use anyhow::Result;

#[derive(Debug)]
struct HandPair_1 {
    value: usize,
}

#[derive(Debug)]
struct HandPair_2 {
    value: usize,
}


// 2 + A(0)
// 2 + B(1)
// 2 + C(2)
const WIN_LOSE_2: [usize; 3] = [0, 3, 6];
const WIN_LOSE: [usize; 3] = [3, 6, 0];
const CHOICE_VALUE: [usize; 3] = [3, 1, 2];
// A                              X  Y  Z
// B                              Z  X  Y
// C                              Y  Z  X

fn to_number(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 2,
        "C" => 1,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("nice input guy")
    }
}

fn to_number_2(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("nice input guy")
    }
}


impl FromStr for HandPair_1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        let o2 = to_number(o);
        let p2 = to_number(p);
        let score = p2 + WIN_LOSE[
            (2 + o2 + p2) % WIN_LOSE.len()
        ];

        return Ok(HandPair_1 { value: score });
    }
}

impl FromStr for HandPair_2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        let o2 = to_number_2(o);
        let p2 = to_number_2(p);
        let score = CHOICE_VALUE[
            (o2 + p2) % CHOICE_VALUE.len()
        ] + WIN_LOSE_2[p2];

        println!("parsed: {} {} -> {} {} = {}", o, p, o2, p2, score);

        return Ok(HandPair_2 { value: score });
    }
}

fn main() -> Result<()> {
    let values: usize = include_str!("input2_1.prod")
        .lines()
        .flat_map(|x| x.parse::<HandPair_2>())
        .map(|x| x.value)
        .sum();

    let mut values = vec![Rc::new(Foo {}), Rc::new(Foo {}), Rc::new(Foo {})];
    let last_foo = values.last().unwrap().clone();

    for x in &values {
    }

    values.pop();

    println!("Daddy {:?}", last_foo);
    return Ok(());
}

