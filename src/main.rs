#![feature(int_abs_diff)]

fn project(s: &str) -> u64 {
    println!("'{}'", s);
    return s
        .chars()
        .map(|x| {
            // step 1 convert char => byte
            // x - 'a'
            // return 10pow(FOO[x - 'a'])
            0x1 << (x as u8 - 'a' as u8)
        })
        .fold(0, |acc, x| acc + x);
}

fn contains(a: u64, b: u64) -> bool {
    return a & b == b;
}

fn missing(a: u64, b: u64) -> u64 {
    (a ^ b).count_ones() as u64
}

fn create_contain(b: u64) -> Box<dyn Fn(&u64) -> bool> {
    return Box::new(move |a| {
        println!("create_contain  :: a: {:#010b} b: {:#010b}", a, b);
        return contains(*a, b);
    });
}

fn create_missing(b: u64, count: u64) -> Box<dyn Fn(&u64) -> bool> {
    return Box::new(move |a| {
        println!("create_missing :: a: {:#010b} b: {:#010b} count {}", a, b, missing(*a, b));
        return missing(*a, b) == count;
    });
}

fn remove(list: Vec<u64>, f: Box<dyn Fn(&u64) -> bool>) -> (Vec<u64>, u64) {
    println!("list {:?}", list);
    let (idx, matched) = list
        .iter()
        .enumerate()
        .filter(|it| {
            return f(it.1);
        })
        .take(1)
        .collect::<Vec<(usize, &u64)>>()[0];

    let mut list = list.clone();
    list.remove(idx);
    return (list, *matched);
}

fn get_value(proj_val: u64, numbers: &[u64; 10]) -> usize {
    let (idx, _) = numbers.iter().enumerate().filter(|it| {
        let (_, x) = it;
        return **x == proj_val;
    }).collect::<Vec<(usize, &u64)>>()[0];

    return idx;
}

fn main() {
    let count = include_str!("day8.input")
        .trim()
        .lines()
        .fold(0, |acc, line| {
            let (input, output) = line.split_once(" | ").unwrap();
            let input = input.split(" ");
            let mut numbers: [u64; 10] = input.clone().fold([0; 10], |mut numbers, seg| {
                match seg.len() {
                    2 => numbers[1] = project(seg),
                    7 => numbers[8] = project(seg),
                    4 => numbers[4] = project(seg),
                    3 => numbers[7] = project(seg),
                    _ => {}
                }
                numbers
            });

            println!("1 : {:#010b}", numbers[1]);
            println!("8 : {:#010b}", numbers[8]);
            println!("4 : {:#010b}", numbers[4]);
            println!("3 : {:#010b}", numbers[7]);

            let group5: Vec<u64> = input.clone().filter(|seg| &seg.len() == &5).map(project).collect();
            let group6: Vec<u64> = input.filter(|seg| &seg.len() == &6).map(project).collect();

            group5.iter().for_each(|x| {
                println!("group5 : {:#010b}", x);
            });
            group6.iter().for_each(|x| {
                println!("group6 : {:#010b}", x);
            });

            let (group6, nine) = remove(group6, create_contain(numbers[4]));
            println!("FOUND NINE : contains 4 {:#010b} U {:#010b}", nine, numbers[4]);
            numbers[9] = nine;

            // find 2
            let (group5, two) = remove(group5, create_missing(numbers[9], 3));
            println!("FOUND TWO : from group5 {:#010b} missing 2 {:#010b}", two, nine);
            numbers[2] = two;

            // find 5
            let (group5, five) = remove(group5, create_missing(numbers[7], 4));
            numbers[5] = five;
            println!("five : {:#010b}", five);
            numbers[3] = group5[0];

            println!("FINDING SIX");
            let (group6, six) = remove(group6, create_contain(numbers[5]));
            numbers[6] = six;
            println!("six : {:#010b}", six);

            println!("remainder from six");
            numbers[0] = group6[0];

            println!("{:?}", numbers);
            let output: Vec<u64> = output
                .split(" ")
                .map(str::trim)
                .map(project)
                .collect();

            let res = output.iter().rev().enumerate().fold(0, |acc, it| {
                let (idx, val) = it;
                let val_idx = get_value(*val, &numbers);
                println!("val: {} :: val_idx {}", val, val_idx);
                return acc + 10_u64.pow(idx as u32) * val_idx as u64;
            });
            return res + acc;
        });

    println!("{} Powered by RustLang:tm:", count);
}
