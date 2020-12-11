use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/6.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut counter = 0;
    let mut counter2 = 0;

    for i in 0..groups.len() {
        let members: Vec<&str> = groups[i].split('\n').collect();

        // let mut yes = Vec::new();
        let mut yes: HashMap<char, usize> = HashMap::new();
        for member in &members {
            for q in member.chars() {
                match yes.get(&q) {
                    None => yes.insert(q, 1),
                    Some(v) => yes.insert(q, v + 1),
                };
            }
        }
        let mut allyes = 0;
        for y in &yes {
            if y.1 == &members.len() {
                allyes += 1
            }
        }

        counter2 += allyes;
        counter += yes.len();
    }

    println!("First: {} Second:{}", counter, counter2);
}
