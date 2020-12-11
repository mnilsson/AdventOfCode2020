use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/3.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let lines: Vec<Vec<char>> = input
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let slopes = [
        [1, 1],
        // [1,3], this is the first slope
        [1, 5],
        [1, 7],
        [2, 1],
    ];

    let first_sum = trees(&lines, 1, 3);

    println!("first {}", first_sum);

    let mut second_sum = first_sum;

    for slope in slopes.iter() {
        second_sum *= trees(&lines, slope[0], slope[1])
    }

    println!("second {}", second_sum);
}

fn trees(lines: &Vec<Vec<char>>, y_inc: usize, x_inc: usize) -> usize {
    let mut counter = 0;
    let mut x = 0;
    let mut y = 0;

    while y < lines.len() {
        let modval = lines[y].len();
        if lines[y][x % modval] == '#' {
            counter += 1
        }
        y += y_inc;
        x += x_inc;
    }
    counter
}
