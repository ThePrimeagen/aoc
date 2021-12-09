#![feature(int_abs_diff)]

fn get_distance_traveled(crabs: &Vec<u64>, position: u64) -> u64 {
    let mut distance = 0;

    crabs.iter().for_each(|x| {
        distance += x.abs_diff(position);
    });

    return distance;
}

fn get_distance_traveled_complex(crabs: &Vec<u64>, position: u64) -> u64 {
    let mut distance = 0;

    crabs.iter().for_each(|x| {
        let d = x.abs_diff(position);
        distance += d * (d + 1) / 2;
    });

    return distance;
}

fn main() {
    let crabs: Vec<u64> = include_str!("day7.test")
        .trim()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let (low, high) = crabs.iter().fold((10000, 0), |mut bounds, crab| {
        if bounds.0 > *crab {
            bounds.0 = *crab;
        }
        if bounds.1 < *crab {
            bounds.1 = *crab;
        }
        return bounds;
    });

    let count = (low..high).map(|x| get_distance_traveled_complex(&crabs, x)).min();
    println!("{:?}", count);
}

