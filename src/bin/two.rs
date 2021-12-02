use std::path::Path;
use std::fs;
use adventofcode2021::two::*;

fn main() {
    let input_path = Path::new("/Users/thomaskunze/Projects/adventofcode2021/src/bin/two_input");
    let input_string = fs::read_to_string(input_path).expect("Could not read file.");
    let mut directions: Vec<Direction> = vec![];

    for line in input_string.lines() {
        let tuple: Vec<&str> = line.split(" ").collect();
        directions.push (match tuple[0] {
            "forward" => Direction::Forward(tuple[1].parse().expect("Could not parse")),
            "up" => Direction::Up(tuple[1].parse().expect("Could not parse")),
            "down" => Direction::Down(tuple[1].parse().expect("Could not parse")),
            _ => return
        });
    }

    let mut submarine = Submarine::new();

    for direction in &directions {
        submarine.move_submarine(direction);
    }

    println!("{}", submarine.multiply_coordinates());
}