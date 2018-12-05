use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1_and_2() {
    let reader = BufReader::new(File::open("resources/input3.txt").expect("Cannot open file.txt"));


    let mut collisons = [false; 1500];

    let mut state = [[0; 1000]; 1000];
    for line in reader.lines() {
        let mut split1 = line.unwrap();
        let split = split1.split(&['#', ' ', ':', ',', 'x'][..]).collect::<Vec<&str>>();

        let x: i32 = split[3].parse().unwrap();
        let y: i32 = split[4].parse().unwrap();

        let width: i32 = split[6].parse().unwrap();
        let height: i32 = split[7].parse().unwrap();

        let line_number: i32 = split[1].parse().unwrap();

        for w_offset in 0..width {
            for h_offset in 0..height {
                let x_index = (x + w_offset) as usize;
                let y_index = (y + h_offset) as usize;
                if state[x_index][y_index] != 0 {
                    if state[x_index][y_index] != -1 {
                        collisons[state[x_index][y_index] as usize] = true;
                    }

                    collisons[line_number as usize] = true;
                    state[x_index][y_index] = -1;
                } else {
                    state[x_index][y_index] = line_number;
                }
            }
        }
    }

    let mut total = 0;
    for x in 0..state.len() {
        for y in 0..state[x].len() {
            if state[x][y] == -1 {
                total += 1;
            }
        }
    }

    println!("total '{}'", total);

    for x in 1..collisons.len() {
        if collisons[x] == false {
            println!("Collison free '{}'", x);
            break;
        }
    }
}
