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


    lines.iter().fold(pivot, |mut p, line| {
        line.chars().zip(&mut p).for_each(|(c, p)| {
            p.push(match c {
                '1' => true,
                '0' => false,
                _ => false,
            });
        });
        p
    })
}

fn filter_by_count<'a>(lines: &Vec<&'a str>, idx: usize, most: bool) -> &'a str {
    if lines.len() == 1 {
        return lines[0];
    }

    let pivot = create_pivot_container(lines);
    let line_len = pivot.get(0).unwrap().len();
    let pivot_line = pivot.
        get(idx).unwrap();

    let most_common = pivot_line.
        iter().filter(|c| **c).count();

    let most_common = if most {
        most_common >= line_len - most_common
    } else {
        most_common < line_len - most_common
    };

    filter_by_count(&pivot.get(idx).unwrap().iter().
        zip(0..line_len).
        fold(Vec::<&'a str>::new(), |mut acc, (v, i)| {
            if *v == most_common {
                acc.push(lines.get(i).unwrap());
            }
            acc
        }), idx + 1, most)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = include_str!("day3.input").lines().collect();

    let oxy_generator = filter_by_count(&lines, 0, true);
    let co2_scrub = filter_by_count(&lines, 0, false);

    let o = u32::from_str_radix(oxy_generator, 2)?;
    let c = u32::from_str_radix(co2_scrub, 2)?;

    println!("{} oxy_generator {} co2  == {};; Built in RustLang:tm:", o, c, o * c);
    /*
    let gamma_vec: Vec<bool> = lines.
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

    */

    return Ok(());
}
