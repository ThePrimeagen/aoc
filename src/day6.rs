fn main() {
    let mut fishes: [u64; 9] = include_str!("day6.input")
        .lines()
        .take(1)
        .flat_map(|x| x.split(","))
        .map(|x| x.parse::<u64>().unwrap())
        .fold([0; 9], |mut fishes, days_until_making_babies| {
            fishes[days_until_making_babies as usize] += 1;
            fishes
        });

    for _ in 0..256 {
        let D = fishes[0];
        fishes[0] = 0;
        for f_idx in 1..9 {
            fishes[f_idx - 1] += fishes[f_idx];
            fishes[f_idx] = 0;
        }
        fishes[8]=D;
        fishes[6]+=D;
    }
    println!("After {}", fishes.iter().fold(0, |acc, x| acc + x));
}

