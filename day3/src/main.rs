extern crate regex;

use std::fs;
use regex::Regex;

#[derive(Debug)]
struct CutSpec {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl CutSpec {
    fn parse(x: &str) -> CutSpec {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(x).unwrap();

        CutSpec{
            id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            left: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            top: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            width: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            height: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    fn does_overlap(&self, other: &CutSpec) -> bool {
        let start_x = self.left;
        let start_y = self.top;
        let end_x = self.left + self.width;
        let end_y = self.top + self.height;

        let other_start_x = other.left;
        let other_start_y = other.top;
        let other_end_x = other.left + other.width;
        let other_end_y = other.top + other.height;

        !(end_x < other_start_x ||
        start_x > other_end_x ||
        end_y < other_start_y ||
        start_y > other_end_y)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to open file");

    let specs: Vec<CutSpec> = input.lines()
        .map( |line| CutSpec::parse(line))
        .collect();

    let board_width: usize = 1000;
    let board_size = board_width * board_width;
    let mut board: Vec<u8> = vec![0; board_size];

    println!("Specs: {:?}", specs.len());
    println!("Board size: {}", board.len());

    for spec in specs.iter() {
        let start_index = board_index(board_width, &spec);

        for i in 0..spec.height {
            for j in start_index..start_index+spec.width {
                let idx: usize = j + i * board_width;
                board[idx] += 1;
            }
        }
    }

    //print_board(&board, board_width);
    let mut total = 0;
    for square in board {
        if square > 1 {
            total += 1;
        }
    }

    println!("Total overlap: {}", total);

    for first in specs.iter() {
        let mut overlap = false;

        for second in specs.iter() {
            if first.id != second.id && first.does_overlap(&second) {
                overlap = true;
                break;
            }
        }

        if !overlap {
            println!("{:?}", first);
        }
    }
}

fn board_index(board_width: usize, spec: &CutSpec) -> usize {
    board_width * spec.top + spec.left
}

fn print_board(board: &Vec<u8>, board_width: usize) {
    for x in (0..board_width*board_width).step_by(board_width) {
        for y in 0..board_width {
            print!("{}", board[x + y]);
        }

        print!("\n");
    }
}
