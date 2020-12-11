use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/11.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);
    let mut nums: Vec<&str> = input.split("\n").collect();

    let floor_height = nums.len();
    let floor_width = nums[0].len();

    let mut floor_vec: Vec<Vec<Option<Chair>>> = vec![vec![None; floor_width]; floor_height];

    for i in 0..nums.len() {
        let row_chars: Vec<char> = nums[i].chars().collect();
        for j in 0..row_chars.len() {
            match row_chars[j] {
                'L' => floor_vec[i][j] = Some(Chair::Empty),
                _ => (),
            }
        }
    }

    let mut floor = Floor {
        width: floor_width,
        height: floor_height,
        floor: floor_vec,
        stopped: false,
    };

    while floor.stopped == false {
        floor.mutate();
        // floor.print();
    }

    println!("{}", floor.count_taken())
}

#[derive(Debug, Clone)]
enum Chair {
    Empty,
    Taken,
}

#[derive(Debug)]
struct Floor {
    width: usize,
    height: usize,
    floor: Vec<Vec<Option<Chair>>>,
    stopped: bool,
}

impl Floor {
    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                match &self.floor[i][j] {
                    Some(Chair::Taken) => print!("#"),
                    Some(Chair::Empty) => print!("L"),
                    None => print!("."),
                    _ => (),
                }
            }
            print!("\n");
        }
        print!("\n");
    }
    fn mutate(&mut self) {
        let mut mutated = 0;
        let mut floor_clone = self.floor.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                match &self.floor[i][j] {
                    Some(chair) => {
                        let taken_neighbors = self.taken_neighbors_far(i, j);
                        match chair {
                            Chair::Empty => {
                                if taken_neighbors == 0 {
                                    mutated += 1;
                                    floor_clone[i][j] = Some(Chair::Taken)
                                }
                            }
                            Chair::Taken => {
                                // if taken_neighbors >= 4 {
                                if taken_neighbors >= 5 {
                                    mutated += 1;
                                    floor_clone[i][j] = Some(Chair::Empty)
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        if mutated == 0 {
            self.stopped = true
        }
        self.floor = floor_clone
    }

    fn taken_neighbors(&self, y: usize, x: usize) -> usize {
        let dirs = [-1, 0, 1];

        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                let yy = y as i32 + dirs[i];
                let xx = x as i32 + dirs[j];

                if (yy as usize == y && xx as usize == x)
                    || yy < 0
                    || yy >= (self.height) as i32
                    || xx < 0
                    || xx >= (self.width) as i32
                {
                } else {
                    match self.floor[yy as usize][xx as usize] {
                        Some(Chair::Taken) => count += 1,
                        _ => (),
                    }
                }
            }
        }
        count
    }

    fn taken_neighbors_far(&self, y: usize, x: usize) -> usize {
        let dirs = [-1, 0, 1];

        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                let mut yy = y as i32 + dirs[i];
                let mut xx = x as i32 + dirs[j];

                let mut not_empty = true;

                while not_empty {
                    if (yy as usize == y && xx as usize == x) || self.out_of_bounds(yy, xx) {
                        not_empty = false
                    } else {
                        match self.floor[yy as usize][xx as usize] {
                            Some(Chair::Taken) => {
                                count += 1;
                                not_empty = false;
                            }
                            Some(Chair::Empty) => not_empty = false,
                            _ => (),
                        }
                    }
                    yy = yy as i32 + dirs[i];
                    xx = xx as i32 + dirs[j];
                }
            }
        }
        count
    }

    fn out_of_bounds(&self, y: i32, x: i32) -> bool {
        if y < 0 || y >= (self.height) as i32 || x < 0 || x >= (self.width) as i32 {
            true
        } else {
            false
        }
    }

    fn count_taken(&self) -> usize {
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                match &self.floor[i][j] {
                    Some(Chair::Taken) => count += 1,
                    _ => (),
                }
            }
        }
        count
    }
}
