use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/13.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let mut lines: Vec<&str> = input.split('\n').collect();
    let earliest: usize = lines[0].parse().unwrap();
    let buses: Vec<usize> = lines[1]
        .split(',')
        .filter(|c| c != &"x")
        .map(|c| c.parse().unwrap())
        .collect();

    let res = run_part1(earliest, buses);

    println!("First: {}", res);

    let buses2t: Vec<&str> = lines[1].split(',').collect();

    let mut buses2: Vec<(usize, usize)> = Vec::new();

    for i in 0..buses2t.len() {
        if buses2t[i] != "x" {
            buses2.push((buses2t[i].parse::<usize>().unwrap(), i));
        }
    }

    let res2 = run_part2(buses2);
    println!("Second: {}", res2);
}

fn run_part1(earliest: usize, buses: Vec<usize>) -> usize {
    let mut lowest = 0;
    let mut lowest_value = 0;
    for bus in buses {
        let m = earliest / bus;
        let m = (m + 1) * bus;
        if lowest == 0 || m < lowest_value {
            lowest_value = m;
            lowest = bus;
        }
    }
    lowest * (lowest_value - earliest)
}

fn run_part2(buses: Vec<(usize, usize)>) -> usize {
    let mut time = 0;
    loop {
        let mut done = true;

        let mut s = 1;
        for bus in &buses {
            if (time + bus.1) % bus.0 == 0 {
                s *= bus.0;
            } else {
                done = false
            }
        }
        if done {
            break;
        }
        time += s;
    }
    time
}
