use anyhow::Result;

enum Event {
    FooEvent(usize),
    BarEvent(String),
}

fn some_f(event: Event) {
    match event {
        Event::FooEvent(_) => todo!(),
        Event::BarEvent(_) => todo!(),
    }
}

fn main() -> Result<()> {
    let mut max: Vec<usize> = include_str!("./input1_1.test")
        .split("\n\n")
        .map(|x| {
            return x
                .lines()
                .flat_map(str::parse::<usize>)
                .sum();
        })
        .collect();


    max.sort_by(|a, b| b.cmp(a));

    println!("max me daddy {:?}", max
         .into_iter()
         .take(3)
         .sum::<usize>()
    );

    return Ok(());
}

