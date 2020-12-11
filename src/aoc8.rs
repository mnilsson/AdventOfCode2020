use nom::lib::std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/8.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);
    let mut mem = Vec::new();
    let ops: Vec<&str> = input.split("\n").collect();
    for op in ops {
        let parts: Vec<&str> = op.split(" ").collect();
        let parsed_op = match parts[0] {
            "nop" => {
                let val = parts[1].parse::<i32>().unwrap();
                Operation::Nop(val)
            }
            "acc" => {
                let val = parts[1].parse::<i32>().unwrap();
                Operation::Acc(val)
            }
            "jmp" => {
                let val = parts[1].parse::<i32>().unwrap();
                Operation::Jmp(val)
            }
            _ => panic!("unknown operation"),
        };
        mem.push(parsed_op);
    }

    let mut mutated = Vec::new();
    let memlen = mem.len();
    let original_mem = mem.clone();
    let mut first_visit_halt = true;
    loop {
        let mut vm = Vm {
            pc: 0,
            acc: 0,
            mem: mem.clone(),
            visited: vec![false; memlen],
            halt_reason: None,
        };

        while vm.running() {
            vm.tick()
        }
        match vm.halt_reason {
            Some(HaltReason::Visit) => {
                if first_visit_halt {
                    println!("First: {}", vm.acc);
                    first_visit_halt = false;
                }
                mem = original_mem.clone();
                if !mutate_memory(&mut mem, &mut mutated) {
                    break;
                }
            }
            Some(HaltReason::MemOverflow) => {
                println!("Second: {}", vm.acc);
                break;
            }
            None => panic!("wat"),
        }
    }
}

fn mutate_memory(mem: &mut Vec<Operation>, mutated: &mut Vec<usize>) -> bool {
    for o in 0..mem.len() {
        if mutated.contains(&o) {
            continue;
        }
        match mem[o] {
            Operation::Jmp(n) => {
                mem[o] = Operation::Nop(n);
                mutated.push(o);
                return true;
            }
            Operation::Nop(n) => {
                mem[o] = Operation::Jmp(n);
                mutated.push(o);
                return true;
            }
            _ => (),
        }
    }
    return false;
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
enum HaltReason {
    Visit,
    MemOverflow,
}
#[derive(Debug)]
struct Vm {
    pc: usize,
    acc: i32,
    mem: Vec<Operation>,
    visited: Vec<bool>,
    halt_reason: Option<HaltReason>,
}

impl Vm {
    fn tick(&mut self) {
        self.visited[self.pc] = true;
        match self.mem[self.pc] {
            Operation::Nop(_) => self.pc += 1,
            Operation::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }
            Operation::Jmp(val) => self.pc = ((self.pc as i32) + val) as usize,
        }
    }

    fn running(&mut self) -> bool {
        if self.pc >= self.mem.len() {
            self.halt_reason = Some(HaltReason::MemOverflow);
            return false;
        }
        if self.visited[self.pc] {
            self.halt_reason = Some(HaltReason::Visit);
            false
        } else {
            true
        }
    }
}
