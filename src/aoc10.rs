use nom::lib::std::collections::BTreeMap;
use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::buffers::RingBuffer;
use std::panic::resume_unwind;
// use std::io::prelude::*;
// use std::collections::VecDeque;

pub fn run() {
    let mut file = File::open("inputs/10.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);
    let mut nums: Vec<u64> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    nums.push(0);
    nums.sort();

    let last = nums.last().unwrap();
    let lastval = last + 3;
    nums.push(last + 3);

    let mut ones = 0;
    let mut threes = 0;

    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        if diff == 1 {
            ones += 1
        } else if diff == 3 {
            threes += 1
        } else {
            panic!("nope")
        }
    }
    println!("First: {}", ones * threes);

    let mut paths: HashMap<u64, u64> = HashMap::new();
    paths.insert(0, 1);

    for n in nums {
        if n == 0 {
            continue;
        }

        paths.insert(
            n,
            paths.get(&(n - 1)).unwrap_or(&0)
                + paths.get(&(n - 2)).unwrap_or(&0)
                + paths.get(&(n - 3)).unwrap_or(&0),
        );
    }

    println!("Second: {}", paths.get(&lastval).unwrap());
}
