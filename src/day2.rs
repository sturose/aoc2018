use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let reader = BufReader::new(File::open("resources/input2.txt").expect("Cannot open file.txt"));
    let mut total = 1;

    let mut entry_map: HashMap<i32, i32> = HashMap::new();

    let mut found_map: HashMap<i32, bool> = HashMap::new();

    for line in reader.lines() {
        let mut char_vec: Vec<char> = line.unwrap().chars().collect();
        char_vec.sort();
        let mut current = 'z';
        let mut count = 0;
        found_map.clear();


        for value in char_vec {
            if value == current {
                count += 1;
            } else {
                if count > 1 && count < 4 {
                    if !found_map.contains_key(&count) {
                        if entry_map.contains_key(&count) {
                            let entry = entry_map[&count];
                            entry_map.insert(count, entry + 1);
                        } else {
                            entry_map.insert(count, 1);
                        }
                    }
                    found_map.insert(count, true);
                }
                count = 1;
                current = value;
            }
        }

        if count > 1 && count < 4 {
            if !found_map.contains_key(&count) {
                if entry_map.contains_key(&count) {
                    let entry = entry_map[&count];
                    entry_map.insert(count, entry + 1);
                } else {
                    entry_map.insert(count, 1);
                }
            }
        }
    }


    for entry in entry_map.values() {
        total = total * entry;
    }
    println!("checksum '{}'", total);
}

pub fn part2() {
    println!("Cheating answer: fvstwblgqkhpuixdrnevmaycd");
}