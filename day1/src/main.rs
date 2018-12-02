use std::fs;

fn main() {
    let contents = fs::read_to_string("frequencies.txt")
        .expect("Unable to open frequencies file");
    
    let values: Vec<i32> = contents
        .lines()
        .map( |line| line.parse::<i32>().unwrap() )
        .collect();

    let result = values
        .iter()
        .fold(0, |acc, x| acc + x);

    println!("{:?}", result);
}
