use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part2() {

    let mut positives = vec![false; 2000000];
    let mut negatives = vec![false; 2000000];
    let mut found_duplicate = false;
    let mut found_first_total = false;
    let mut duplicate = 0;
    let mut total: i32 = 0;
    let mut first_total: i32 = 0;

    let mut count: i32 = 0;
    while found_duplicate == false {
        let reader = BufReader::new(File::open("resources/input1.txt").expect("Cannot open file.txt"));
        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                let value: i32 = word.parse().unwrap();
                total += value;
                let index: usize = total.abs() as usize;
                if total < 0 {
                    if negatives[index] == true {
                        if !found_duplicate {
                            found_duplicate = true;
                            duplicate = total
                        }
                    }
                    negatives[index] = true
                }
                if total >= 0 {
                    if positives[index] == true {
                        if !found_duplicate {
                            found_duplicate = true;
                            duplicate = total
                        }
                    }
                    positives[index] = true
                }
            }
        }
        if !found_first_total {
            found_first_total = true;
            first_total = total;
        }
        count += 1;
    }

    println!("Loops where dup was found '{}'", count);
    println!("Total '{}'", first_total);
    println!("Duplicate '{}'", duplicate);
}