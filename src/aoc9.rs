use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::buffers::RingBuffer;

pub fn run() {
    let mut file = File::open("inputs/9.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);
    let mut nums: Vec<u64> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .rev()
        .collect();

    let mut nums_copy = nums.clone();
    nums_copy.reverse();

    let look_size = 25;

    let mut buffer = RingBuffer::new(look_size);

    for _ in 0..look_size {
        let num = nums.pop();
        match num {
            Some(n) => buffer.push(n),
            None => panic!("noo"),
        }
    }

    let mut the_num = 0;
    while nums.len() > 0 {
        let num = nums.pop();

        match num {
            Some(n) => {
                if is_sum_of_two(n, buffer.get_data()) {
                    buffer.push(n)
                } else {
                    the_num = n;
                    println!("First: {}", n);
                    break;
                }
            }
            None => panic!("noooo!"),
        }
    }

    let mut found = false;
    for i in 0..nums_copy.len() {
        let mut asum = 0;
        let mut arr = Vec::new();
        for j in i..nums_copy.len() {
            if asum < the_num && nums_copy[j] != the_num {
                asum += nums_copy[j];
                arr.push(nums_copy[j]);
            }

            if asum > the_num {
                break;
            }
            if asum == the_num {
                found = true;
                arr.sort();

                println!("Second: {}", arr.first().unwrap() + arr.last().unwrap());
                break;
            }
        }
    }
}

fn is_sum_of_two(num: u64, v: Vec<u64>) -> bool {
    for i in 0..(v.len()) {
        for j in i..(v.len()) {
            if v[i] + v[j] == num {
                return true;
            }
        }
    }
    false
}
