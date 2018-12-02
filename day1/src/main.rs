use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("frequencies.txt")
        .expect("Unable to open frequencies file");
    
    let values: Vec<i32> = contents
        .lines()
        .map( |line| line.parse::<i32>().unwrap() )
        .collect();

    println!("Part One: {}", part_one(&values));
    println!("Part Two: {}", part_two(&values));
}

fn part_one(values: &Vec<i32>) -> i32 {
    values.iter().fold(0, |acc, x| acc + x)
}

fn part_two(values: &Vec<i32>) -> i32 {
    let mut dup_detection: HashSet<i32> = HashSet::new();
    let mut found = false;
    let mut freq = 0;

    dup_detection.insert(freq);

    while !found {
        for value in values {
            freq += value;

            if dup_detection.contains(&freq) {
                found = true;
                break;
            } else {
                dup_detection.insert(freq.clone());
            }
        }
    }

    freq
}
