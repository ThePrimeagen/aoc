use anyhow::Result;

fn main() -> Result<()> {
    let mut max = include_str!("./input1_1.test")
        .split("\n\n")
        .map(|x| {
            return x
                .lines()
                .flat_map(str::parse::<usize>)
                .sum::<usize>();
        })
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));

    println!("max me daddy {:?}", max
         .into_iter()
         .take(3)
         .sum::<usize>()
    );

    return Ok(());
}

