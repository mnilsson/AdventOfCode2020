use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/12.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let mut lines: Vec<&str> = input.split('\n').collect();

    let res = run_moves(&mut lines, 0, 0, 1, 0, false);

    println!("First: {}", res.0.abs() + res.1.abs());

    let res = run_moves(&mut lines, 0, 0, 10, -1, true);

    println!("Second: {}", res.0.abs() + res.1.abs());
}

fn run_moves(lines: &mut Vec<&str>, x: i32, y: i32, wx: i32, wy: i32, move_wp: bool) -> (i32, i32) {
    let mut y = y;
    let mut x = x;
    let mut wx = wx;
    let mut wy = wy;
    for line in lines {
        let num: i32 = line[1..].parse().unwrap();
        let chr = line.chars().nth(0).unwrap();

        let (mut my, mut mx) = if move_wp { (wy, wx) } else { (y, x) };
        match chr {
            'N' => my -= num,
            'S' => my += num,
            'E' => mx += num,
            'W' => mx -= num,
            _ => (),
        }
        if move_wp {
            wy = my;
            wx = mx;
        } else {
            y = my;
            x = mx;
        }

        match chr {
            'R' | 'L' => {
                for _ in 0..(num / 90) {
                    let tx = wx;
                    let ty = wy;
                    if chr == 'R' {
                        wx = -ty;
                        wy = tx;
                    } else {
                        wx = ty;
                        wy = -tx;
                    }
                }
            }
            'F' => {
                y += wy * num;
                x += wx * num;
            }
            _ => (),
        };
    }

    (x, y)
}
