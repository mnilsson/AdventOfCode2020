use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/1.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let numbers: Vec<u32> = input
        .split('\n')
        .rev()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("First: {}", numbers[i] * numbers[j]);
                break;
            }
        }
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("Second: {}", numbers[i] * numbers[j] * numbers[k])
                }
            }
        }
    }
}
