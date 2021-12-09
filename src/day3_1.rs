
fn create_pivot_container(lines: &Vec<&str>) -> Vec<Vec<bool>> {
    let mut pivot: Vec<Vec<bool>> = vec![];
    match lines.first() {
        Some(line) => {
            for _ in 0..line.len() {
                pivot.push(vec![]);
            }
        },
        None => panic!("There wasn't an input file"),
    };

    return pivot;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = include_str!("day3.input").lines().collect();
    let pivot = create_pivot_container(&lines);

    let gamma_vec: Vec<bool> = lines.
        iter().
        fold(pivot, |mut p, line| {
            line.chars().zip(&mut p).for_each(|(c, p)| {
                p.push(match c {
                    '1' => true,
                    '0' => false,
                    _ => false,
                });
            });
            p
        }).
        iter().
        map(|line| {
            let count = line.iter().filter(|c| **c).count();
            count > line.len() / 2
        }).
        collect();

    let eps_vec: Vec<bool> = gamma_vec.
        iter().
        map(|c| !c).
        collect();

    let g = u32::from_str_radix(
        &gamma_vec.iter().map(|one| if *one { "1" } else { "0" }).collect::<Vec<&str>>().join(""), 2)?;

    let e = u32::from_str_radix(
        &eps_vec.iter().map(|one| if *one { "1" } else { "0" }).collect::<Vec<&str>>().join(""), 2)?;

    println!("gamma {} eps {} didn't kill himself = {} Built in RustLang:tm:", g, e, g * e);

    return Ok(());
}
