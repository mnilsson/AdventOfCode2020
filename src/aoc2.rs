use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/2.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let lines: Vec<Vec<&str>> = input
        .split('\n')
        .map(|row| row.split(' ').collect::<Vec<&str>>())
        .collect();

    let mut counter = 0;
    let mut counter2 = 0;
    for pass in lines {
        let range: Vec<u32> = pass[0].split('-').map(|s| s.parse().unwrap()).collect();
        let chr = pass[1].chars().rev().last().unwrap();
        let password = pass[2];

        let mut passcounter = 0;
        for passchar in password.chars() {
            if passchar == chr {
                passcounter += 1;
            }
        }

        if passcounter >= range[0] && passcounter <= range[1] {
            counter += 1
        }

        // Second part
        let passchars2: Vec<char> = password.chars().collect();

        if (passchars2[range[0] as usize - 1] == chr && passchars2[range[1] as usize - 1] != chr)
            || (passchars2[range[0] as usize - 1] != chr
                && passchars2[range[1] as usize - 1] == chr)
        {
            counter2 += 1;
        }
    }

    println!("First: {} Second:{}", counter, counter2);
}
