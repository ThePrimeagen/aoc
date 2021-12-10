#![feature(int_abs_diff)]

fn main() {
    let data: Vec<Vec<i32>> = include_str!("day9.input")
        .trim()
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).expect("your mother") as i32)
            .collect())
        .collect();
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut sum: i32 = 0;
    for w in 0..width {
        for h in 0..height {
            if dirs.iter().fold(true, |lowest, dir| {
                let dw = w + dir.0;
                let dh = h + dir.1;
                if dw < 0 || dw >= width ||
                    dh < 0 || dh >= height {
                    return lowest;
                }

                return lowest && data[h as usize][w as usize] < data[dh as usize][dw as usize];
            }) {
                sum += data[h as usize][w as usize] + 1;
            }
        }
    }
    println!("{} Powered by RustLang:tm:", sum);
}

