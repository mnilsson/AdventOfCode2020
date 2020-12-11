use nom::lib::std::collections::HashMap;
use nom::lib::std::mem::take;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/5.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let rows: Vec<&str> = input.split("\n").collect();

    let mut highest = 0;

    let mut taken_ids: HashMap<usize, usize> = HashMap::new();
    for row in rows {
        let id = parse_row(row);
        taken_ids.insert(id, id);
        if id > highest {
            highest = id;
        }
    }

    let mut my_seat = 0;
    for i in 0..highest {
        if !taken_ids.contains_key(&i) {
            my_seat = i;
        }
    }
    println!("First: {}", my_seat);
    println!("Second: {}", highest);
}

fn parse_row(input: &str) -> usize {
    let row_parts = &input[0..7];
    let col_parts = &input[7..10];

    let mut rows = vec![0; 128];
    for i in 0..rows.len() {
        rows[i] = i;
    }
    let row_chars: Vec<char> = row_parts.chars().collect();
    for i in 0..row_chars.len() {
        let high = match &row_chars[i] {
            'F' => false,
            'B' => true,
            _ => panic!(),
        };

        rows = split_and_get(&rows, high)
    }
    let row = rows.first().unwrap();

    let mut cols = vec![0; 8];
    for i in 0..cols.len() {
        cols[i] = i;
    }
    let col_chars: Vec<char> = col_parts.chars().collect();
    for i in 0..col_chars.len() {
        let high = match &col_chars[i] {
            'L' => false,
            'R' => true,
            _ => panic!(),
        };

        cols = split_and_get(&cols, high)
    }
    let col = cols.first().unwrap();

    // println!("{}, {}", row, col);

    let id = (row * 8) + col;
    id
}

fn split_and_get(rows: &Vec<usize>, high: bool) -> Vec<usize> {
    let split_at = rows.len() / 2;

    let split = rows.split_at(split_at);

    if high {
        split.1.to_vec()
    } else {
        split.0.to_vec()
    }
}
