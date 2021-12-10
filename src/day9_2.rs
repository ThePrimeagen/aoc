#![feature(int_abs_diff)]

fn in_bounds(data: &Vec<Vec<u32>>, h: i32, w: i32) -> bool {
    let width = data[0].len() as i32;
    let height = data.len() as i32;

    return w >= 0 && w < width && h >= 0 && h < height;
}

fn is_walkable(data: &Vec<Vec<u32>>, h: i32, w: i32) -> bool {
    return in_bounds(data, h, w) && data[h as usize][w as usize] < 9;
}

fn walk(data: &mut Vec<Vec<u32>>, h: i32, w: i32) -> u32 {
    if !is_walkable(data, h, w) {
        return 0;
    }

    data[h as usize][w as usize] = 9;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    return 1 + dirs.iter().fold(0, |count, (_h, _w)| {
        return count + walk(data, h + _h, w + _w);
    });
}

fn main() {
    let mut data: Vec<Vec<u32>> = include_str!("day9.input")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("your mother"))
                .collect()
        })
        .collect();

    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let mut basins: Vec<u32> = vec![];

    for w in 0..width {
        for h in 0..height {
            basins.push(walk(&mut data, h, w));
        }
    }

    basins.sort();

    println!(
        "{:?} Powered by RustLang:tm:",
        basins.iter().rev().take(3).product::<u32>()
    );
}

