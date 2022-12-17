
use anyhow::Result;

#[macro_export]
macro_rules! tree_me_daddy {
    ( $x:ident, $y:ident, $dir_value:ident, $dir_max:ident, $seen:ident ) => {
        {

            if $dir_value > $dir_max {
                $seen[$y][$x] += 1;
                $dir_max = $dir_value;
            }
        }
    };
}

macro_rules! tree_me_daddy_2 {
    ( $x:ident, $y:ident, $height:ident, $trees:ident, $out:ident ) => {
        {
            if $trees[$y][$x] < $height {
                $out += 1;
            } else if $trees[$y][$x] == $height {
                $out += 1;
                break;
            } else {
                break;
            }

        }
    };
}

fn see(trees: &Vec<Vec<isize>>, x: usize, y: usize) -> usize {
    let w = trees[0].len();
    let h = trees.len();

    let height = trees[y][x];
    let mut out = 1;

    let mut temp = 0;
    for x in (0..x).rev() {
        tree_me_daddy_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for x in x + 1..w {
        tree_me_daddy_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for y in (0..y).rev() {
        tree_me_daddy_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for y in y + 1..h {
        tree_me_daddy_2!(x, y, height, trees, temp);
    }
    out *= temp;

    return out;
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./src/bin/day8.prod")?;

    let trees = file
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            return line
                .chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as isize)
                .collect::<Vec<isize>>();
        })
        .collect::<Vec<Vec<isize>>>();

    let h = trees.len();
    let w = trees[0].len();
    let mut seen = vec![vec![0isize; w]; h];

    /*
    for y in 0..h {
        let mut e_h = -1;
        let mut w_h = -1;
        for x in 0..w {
            let w_idx = w - x - 1;
            let west = trees[y][w_idx];
            let east = trees[y][x];

            tree_me_daddy!(x, y, east, e_h, seen);
            tree_me_daddy!(w_idx, y, west, w_h, seen);
        }
    }

    for x in 0..w {
        let mut n_h = -1;
        let mut s_h = -1;
        for y in 0..h {
            let n_idx = h - y - 1;
            let south = trees[y][x];
            let north = trees[n_idx][x];

            tree_me_daddy!(x, y, south, s_h, seen);
            tree_me_daddy!(x, n_idx, north, n_h, seen);
        }
    }
    */

    for y in 0..h {
        for x in 0..w {
            seen[y][x] = see(&trees, x, y) as isize;
        }
    }

    println!("{:?}", seen.iter().flat_map(|x| x.iter()).max());

    return Ok(());
}

