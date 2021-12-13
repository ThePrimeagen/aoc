#![feature(int_abs_diff)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

enum Axis {
    X(usize),
    Y(usize),
}

impl FromStr for Point {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = &s.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        return Ok(Point { x, y });
    }
}

impl FromStr for Axis {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = &s
            .split_once("fold along ")
            .unwrap()
            .1
            .split_once("=")
            .unwrap();
        let value = value.parse().unwrap();

        return if *axis == "y" {
            Ok(Axis::Y(value))
        } else {
            Ok(Axis::X(value))
        };
    }
}

fn fold(board: Vec<Vec<usize>>, fold: &Axis) -> Vec<Vec<usize>> {
    let mut max_y: usize = board.len();
    let mut max_x: usize = board[0].len();
    let mut next_start_x = 0;
    let mut next_start_y = 0;
    let mut y_fold = false;

    match fold {
        Axis::Y(y) => {
            max_y = *y;
            next_start_y = *y + 1
        }
        Axis::X(x) => {
            max_x = *x;
            next_start_x = *x + 1
        }
    }

    println!("fold max_x {}", max_x);
    println!("fold may_y {}", max_y);

    // copy first board
    let mut new_board = vec![vec![0; max_x]; max_y];
    for y in 0..max_y {
        for x in 0..max_x {
            new_board[y][x] = board[y][x];
        }
    }

    for y in next_start_y..board.len() {
        for x in next_start_x..board[0].len() {

            if board[y][x] == 1 {
                let mut y = y - next_start_y;
                let mut x = x - next_start_x;

                if y_fold {
                    y = new_board.len() - y - 1;
                } else {
                    x = new_board[0].len() - x - 1;
                }

                new_board[y][x] = 1;
            }
        }
    }

    return new_board;
}

fn print_board(board: &Vec<Vec<usize>>) {
    println!("Bounds ({} {})", board[0].len(), board.len());
    for line in board {
        for x in line {
            print!("{}", if *x == 1 { "#" } else { " " });
        }
       println!();
    }
}
fn main() {
    let (points, folds) = include_str!("day13.input")
        .trim()
        .split_once("\n\n")
        .unwrap();

    let points: Vec<Point> = points.lines().map(str::parse).map(Result::unwrap).collect();

    let folds: Vec<Axis> = folds.lines().map(str::parse).map(Result::unwrap).collect();

    let (x_max, y_max) = points
        .iter()
        .fold((0, 0), |(x, y), point| (point.x.max(x), point.y.max(y)));

    println!("Max Points {} {}", x_max, y_max);
    let mut board: Vec<Vec<usize>> = vec![vec![0; x_max + 1]; y_max + 1];

    for p in &points {
        board[p.y][p.x] = 1;
    }

    folds.iter().fold(board, |board, _fold| {
        let board = fold(board, _fold);

        print_board(&board);

        return board;
    });

}
